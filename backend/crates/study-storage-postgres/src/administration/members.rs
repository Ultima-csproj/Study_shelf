mod write;
use super::{
    Administration, Page, PageRequest,
    access::{self, AdminOrigin},
    guard,
};
use crate::{account::session, error::map_sqlx_error};
use serde::Serialize;
use sqlx::{FromRow, PgConnection};
use study_core::{
    account::{AccountError, AccountResult},
    institution::{AcademicRole, InstitutionPermission, MembershipState},
};
use write::apply;

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MembershipRow {
    pub user_id: i64,
    pub institution_id: i64,
    pub user_name: String,
    pub institution_name: String,
    pub state: String,
    pub version: i64,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
pub struct MembershipChange {
    pub state: Option<MembershipState>,
    pub roles: Option<Vec<AcademicRole>>,
    pub permissions: Option<Vec<InstitutionPermission>>,
}
pub struct MembershipDecision<'a> {
    pub expected: Option<i64>,
    pub role: AcademicRole,
    pub accept: bool,
    pub reason: &'a str,
}
pub(crate) async fn for_user(
    conn: &mut PgConnection,
    user: i64,
) -> AccountResult<Vec<MembershipRow>> {
    Ok(sqlx::query_as::<_, MembershipRow>(
        r#"
            SELECT
              m.user_id,
              m.institution_id,
              u.display_name AS user_name,
              i.name AS institution_name,
              m.state,
              m.edit_version AS version,
              ARRAY(
                SELECT
                  role
                FROM
                  membership_academic_roles
                WHERE
                  user_id = m.user_id
                  AND institution_id = m.institution_id
                ORDER BY
                  role
              ) AS roles,
              ARRAY(
                SELECT
                  permission
                FROM
                  membership_permissions
                WHERE
                  user_id = m.user_id
                  AND institution_id = m.institution_id
                ORDER BY
                  permission
              ) AS permissions
            FROM
              institution_memberships m
              JOIN users u ON u.id = m.user_id
              JOIN institutions i ON i.id = m.institution_id
            WHERE
              m.user_id = $1
            ORDER BY
              i.id
        "#,
    )
    .bind(user)
    .fetch_all(conn)
    .await
    .map_err(map_sqlx_error)?)
}
impl Administration {
    pub async fn members(
        &self,
        digest: &[u8],
        institution: i64,
        p: &PageRequest,
    ) -> AccountResult<Page<MembershipRow>> {
        p.validate()?;
        if p.state
            .as_deref()
            .is_some_and(|s| !matches!(s, "active" | "pending" | "rejected" | "revoked"))
        {
            return Err(AccountError::Validation("Некорректное членство"));
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        actor.institution(&mut tx, institution, false).await?;
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  institution_memberships m
                  JOIN users u ON u.id = m.user_id
                WHERE
                  m.institution_id = $1
                  AND (
                    $2 = ''
                    OR u.display_name ILIKE '%' || $2 || '%'
                    OR u.id::text = $2
                  )
                  AND (
                    $3::text IS NULL
                    OR m.state = $3
                  )
            "#,
        )
        .bind(institution)
        .bind(&p.query)
        .bind(&p.state)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, MembershipRow>(
            r#"
                SELECT
                  m.user_id,
                  m.institution_id,
                  u.display_name AS user_name,
                  i.name AS institution_name,
                  m.state,
                  m.edit_version AS version,
                  ARRAY(
                    SELECT
                      role
                    FROM
                      membership_academic_roles
                    WHERE
                      user_id = m.user_id
                      AND institution_id = m.institution_id
                    ORDER BY
                      role
                  ) AS roles,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      membership_permissions
                    WHERE
                      user_id = m.user_id
                      AND institution_id = m.institution_id
                    ORDER BY
                      permission
                  ) AS permissions
                FROM
                  institution_memberships m
                  JOIN users u ON u.id = m.user_id
                  JOIN institutions i ON i.id = m.institution_id
                WHERE
                  m.institution_id = $1
                  AND (
                    $2 = ''
                    OR u.display_name ILIKE '%' || $2 || '%'
                    OR u.id::text = $2
                  )
                  AND (
                    $3::text IS NULL
                    OR m.state = $3
                  )
                ORDER BY
                  m.user_id
                LIMIT
                  $4
                OFFSET
                  $5
            "#,
        )
        .bind(institution)
        .bind(&p.query)
        .bind(&p.state)
        .bind(i64::from(p.size))
        .bind(p.offset())
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }
    pub async fn membership(
        &self,
        origin: AdminOrigin<'_>,
        user: i64,
        institution: i64,
        expected: Option<i64>,
        change: MembershipChange,
        reason: &str,
    ) -> AccountResult<()> {
        guard::reason(reason)?;
        let (mut tx, actor) = self.begin(origin).await?;
        // Global institution staff may appoint the initial manager of an existing institution.
        let empty: bool = sqlx::query_scalar(
            r#"
                SELECT
                  NOT EXISTS (
                    SELECT
                      1
                    FROM
                      institution_memberships m
                      JOIN users u ON u.id = m.user_id
                      JOIN membership_permissions p USING (user_id, institution_id)
                    WHERE
                      m.institution_id = $1
                      AND m.state = 'active'
                      AND u.state = 'active'
                      AND p.permission = 'manage_members'
                  )
            "#,
        )
        .bind(institution)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let initial = empty
            && actor.has("manage_institutions")
            && change.state == Some(MembershipState::Active)
            && change
                .permissions
                .as_ref()
                .is_some_and(|p| p.contains(&InstitutionPermission::ManageMembers));
        if !initial {
            actor.institution(&mut tx, institution, true).await?;
        }
        apply(&mut tx, &actor, user, institution, expected, change, reason).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn decide_membership(
        &self,
        origin: AdminOrigin<'_>,
        user: i64,
        institution: i64,
        decision: MembershipDecision<'_>,
    ) -> AccountResult<()> {
        let MembershipDecision {
            expected,
            role,
            accept,
            reason,
        } = decision;
        guard::reason(reason)?;
        let (mut tx, actor) = self.begin(origin).await?;
        actor.institution(&mut tx, institution, true).await?;
        // Ordinary membership requests lock User before Membership; keep the same order.
        let target: Option<i64> = sqlx::query_scalar(
            r#"
                SELECT
                  id
                FROM
                  users
                WHERE
                  id = $1
                FOR UPDATE
            "#,
        )
        .bind(user)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if target.is_none() {
            return Err(AccountError::NotFound);
        }
        let state: Option<String> = sqlx::query_scalar(
            r#"
                SELECT
                  state
                FROM
                  institution_memberships
                WHERE
                  user_id = $1
                  AND institution_id = $2
                FOR UPDATE
            "#,
        )
        .bind(user)
        .bind(institution)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        match state.as_deref() {
            None => return Err(AccountError::NotFound),
            Some("pending") => {}
            _ => return Err(AccountError::AlreadyDecided),
        }
        apply(
            &mut tx,
            &actor,
            user,
            institution,
            expected,
            MembershipChange {
                state: Some(if accept {
                    MembershipState::Active
                } else {
                    MembershipState::Rejected
                }),
                roles: Some(if accept { vec![role] } else { vec![] }),
                permissions: Some(vec![]),
            },
            reason,
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
