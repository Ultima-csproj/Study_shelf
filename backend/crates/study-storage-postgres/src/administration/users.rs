use super::{
    Administration, Page, PageRequest,
    access::{self, AdminOrigin},
    audit::{self, AuditEvent},
    guard::{self, LastManagers},
};
use crate::{account::session, error::map_sqlx_error};
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;
use study_core::{
    account::{AccountError, AccountResult},
    identity::PlatformPermission,
};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserRow {
    pub id: i64,
    pub name: String,
    pub state: String,
    pub version: i64,
    pub email: Option<String>,
    pub permissions: Vec<String>,
}
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ContributorRow {
    pub id: i64,
    pub name: String,
}
#[derive(Serialize)]
pub struct UserDetail {
    pub user: UserRow,
    pub memberships: Vec<super::MembershipRow>,
    pub contributors: Vec<ContributorRow>,
}
fn can_list(actor: &super::access::Access) -> AccountResult<()> {
    if !actor.has("manage_accounts") && !actor.has("manage_platform_permissions") {
        return Err(AccountError::Forbidden);
    }
    Ok(())
}
impl Administration {
    pub async fn users(&self, digest: &[u8], p: &PageRequest) -> AccountResult<Page<UserRow>> {
        p.validate()?;
        if p.state
            .as_deref()
            .is_some_and(|s| !matches!(s, "active" | "disabled"))
        {
            return Err(AccountError::Validation("Некорректный статус аккаунта"));
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        can_list(&actor)?;
        let private = actor.has("view_account_private_data");
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  users u
                  LEFT JOIN user_credentials c ON c.user_id = u.id
                WHERE
                  (
                    $1 = ''
                    OR u.display_name ILIKE '%' || $1 || '%'
                    OR u.id::text = $1
                    OR (
                      $2
                      AND c.email ILIKE '%' || $1 || '%'
                    )
                  )
                  AND (
                    $3::text IS NULL
                    OR u.state = $3
                  )
            "#,
        )
        .bind(&p.query)
        .bind(private)
        .bind(&p.state)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, UserRow>(
            r#"
                SELECT
                  u.id,
                  u.display_name AS name,
                  u.state,
                  u.admin_version AS version,
                  CASE
                    WHEN $2 THEN c.email
                    ELSE NULL
                  END AS email,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      user_platform_permissions
                    WHERE
                      user_id = u.id
                    ORDER BY
                      permission
                  ) AS permissions
                FROM
                  users u
                  LEFT JOIN user_credentials c ON c.user_id = u.id
                WHERE
                  (
                    $1 = ''
                    OR u.display_name ILIKE '%' || $1 || '%'
                    OR u.id::text = $1
                    OR (
                      $2
                      AND c.email ILIKE '%' || $1 || '%'
                    )
                  )
                  AND (
                    $3::text IS NULL
                    OR u.state = $3
                  )
                ORDER BY
                  u.id
                LIMIT
                  $4
                OFFSET
                  $5
            "#,
        )
        .bind(&p.query)
        .bind(private)
        .bind(&p.state)
        .bind(i64::from(p.size))
        .bind(p.offset())
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }
    pub async fn user(&self, digest: &[u8], id: i64) -> AccountResult<UserDetail> {
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        can_list(&actor)?;
        let user = sqlx::query_as::<_, UserRow>(
            r#"
                SELECT
                  u.id,
                  u.display_name AS name,
                  u.state,
                  u.admin_version AS version,
                  CASE
                    WHEN $2 THEN c.email
                    ELSE NULL
                  END AS email,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      user_platform_permissions
                    WHERE
                      user_id = u.id
                    ORDER BY
                      permission
                  ) AS permissions
                FROM
                  users u
                  LEFT JOIN user_credentials c ON c.user_id = u.id
                WHERE
                  u.id = $1
            "#,
        )
        .bind(id)
        .bind(actor.has("view_account_private_data"))
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?
        .ok_or(AccountError::NotFound)?;
        let memberships = super::members::for_user(&mut tx, id).await?;
        let contributors = sqlx::query_as::<_, ContributorRow>(
            r#"
                SELECT
                  id,
                  display_name AS name
                FROM
                  contributors
                WHERE
                  user_id = $1
                ORDER BY
                  id
            "#,
        )
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(UserDetail {
            user,
            memberships,
            contributors,
        })
    }
    pub async fn set_user_state(
        &self,
        origin: AdminOrigin<'_>,
        id: i64,
        expected: i64,
        active: bool,
        reason: &str,
    ) -> AccountResult<()> {
        guard::reason(reason)?;
        let (mut tx, actor) = self.begin(origin).await?;
        actor.need("manage_accounts")?;
        let old: Option<(String, i64)> = sqlx::query_as(
            r#"
                SELECT
                  state,
                  admin_version
                FROM
                  users
                WHERE
                  id = $1
                FOR UPDATE
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let (old, version) = old.ok_or(AccountError::NotFound)?;
        if version != expected {
            return Err(AccountError::VersionConflict);
        }
        let last = LastManagers::capture(&mut tx, id).await?;
        let state = if active { "active" } else { "disabled" };
        sqlx::query(
            r#"
                UPDATE users
                SET
                  state = $1,
                  admin_version = admin_version + 1
                WHERE
                  id = $2
            "#,
        )
        .bind(state)
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if !active {
            sqlx::query(
                r#"
                    UPDATE user_sessions
                    SET
                      revoked_at = clock_timestamp()
                    WHERE
                      user_id = $1
                      AND revoked_at IS NULL
                "#,
            )
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        }
        last.check(&mut tx).await?;
        audit::write(
            &mut tx,
            &actor,
            AuditEvent {
                kind: "user",
                id,
                institution: None,
                action: "set_user_state",
                reason,
                before: json!({"state":old}),
                after: json!({"state":state}),
            },
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn revoke_sessions(
        &self,
        origin: AdminOrigin<'_>,
        id: i64,
        reason: &str,
    ) -> AccountResult<()> {
        guard::reason(reason)?;
        let (mut tx, actor) = self.begin(origin).await?;
        actor.need("manage_accounts")?;
        let found: Option<i64> = sqlx::query_scalar(
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
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if found.is_none() {
            return Err(AccountError::NotFound);
        }
        let count = sqlx::query(
            r#"
                UPDATE user_sessions
                SET
                  revoked_at = clock_timestamp()
                WHERE
                  user_id = $1
                  AND revoked_at IS NULL
            "#,
        )
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?
        .rows_affected();
        audit::write(
            &mut tx,
            &actor,
            AuditEvent {
                kind: "user",
                id,
                institution: None,
                action: "revoke_user_sessions",
                reason,
                before: json!({}),
                after: json!({"revoked":count}),
            },
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn platform_permissions(
        &self,
        origin: AdminOrigin<'_>,
        id: i64,
        expected: Option<i64>,
        permissions: &[PlatformPermission],
        reason: &str,
    ) -> AccountResult<()> {
        guard::reason(reason)?;
        let (mut tx, actor) = self.begin(origin).await?;
        actor.need("manage_platform_permissions")?;
        let version: Option<i64> = sqlx::query_scalar(
            r#"
                SELECT
                  admin_version
                FROM
                  users
                WHERE
                  id = $1
                FOR UPDATE
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let version = version.ok_or(AccountError::NotFound)?;
        if expected.is_some_and(|v| v != version) {
            return Err(AccountError::VersionConflict);
        }
        let last = LastManagers::capture(&mut tx, id).await?;
        let before: Vec<String> = sqlx::query_scalar(
            r#"
                SELECT
                  permission
                FROM
                  user_platform_permissions
                WHERE
                  user_id = $1
                ORDER BY
                  permission
            "#,
        )
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let mut values: Vec<&str> = permissions.iter().map(|p| p.as_db_value()).collect();
        values.sort_unstable();
        values.dedup();
        sqlx::query(
            r#"
                DELETE FROM user_platform_permissions
                WHERE
                  user_id = $1
            "#,
        )
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        sqlx::query(
            r#"
                INSERT INTO
                  user_platform_permissions (user_id, permission)
                SELECT
                  $1,
                  p
                FROM
                  unnest($2::text[]) p
            "#,
        )
        .bind(id)
        .bind(&values)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        sqlx::query(
            r#"
                UPDATE users
                SET
                  admin_version = admin_version + 1
                WHERE
                  id = $1
            "#,
        )
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        last.check(&mut tx).await?;
        audit::write(
            &mut tx,
            &actor,
            AuditEvent {
                kind: "user",
                id,
                institution: None,
                action: "set_platform_permissions",
                reason,
                before: json!({"permissions":before}),
                after: json!({"permissions":values}),
            },
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
