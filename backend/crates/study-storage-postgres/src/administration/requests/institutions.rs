use super::super::{
    Administration, Page, PageRequest,
    access::{self, AdminOrigin},
    audit::{self, AuditEvent},
    guard,
};
use crate::{account::session, error::map_sqlx_error};

use serde_json::json;

use study_core::account::{AccountError, AccountResult};

use super::{InstitutionRequestInput, RequestRow, validate};
impl Administration {
    pub async fn request_institution(
        &self,
        digest: &[u8],
        input: &InstitutionRequestInput,
    ) -> AccountResult<i64> {
        guard::reason(&input.evidence)?;
        if input.name.trim().is_empty()
            || input.name.chars().count() > 200
            || input.description.chars().count() > 2000
        {
            return Err(AccountError::Validation("Некорректное описание учреждения"));
        }
        let (mut tx, actor) = self.begin(AdminOrigin::Session(digest)).await?;
        let pending: bool = sqlx::query_scalar(
            r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      institution_registration_requests
                    WHERE
                      user_id = $1
                      AND state = 'pending'
                  )
            "#,
        )
        .bind(actor.user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if pending {
            return Err(AccountError::PendingInstitutionRequest);
        }
        let id = sqlx::query_scalar(
            r#"
                INSERT INTO
                  institution_registration_requests (user_id, name, description, evidence)
                VALUES
                  ($1, $2, $3, $4)
                RETURNING
                  id
            "#,
        )
        .bind(actor.user)
        .bind(input.name.trim())
        .bind(input.description.trim())
        .bind(input.evidence.trim())
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(id)
    }
    pub async fn own_institution_requests(
        &self,
        digest: &[u8],
        p: &PageRequest,
    ) -> AccountResult<Page<RequestRow>> {
        p.validate()?;
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  institution_registration_requests
                WHERE
                  user_id = $1
            "#,
        )
        .bind(actor.user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, RequestRow>(
            r#"
                SELECT
                  r.id,
                  r.user_id,
                  u.display_name AS user_name,
                  r.name,
                  r.description,
                  r.evidence,
                  r.state,
                  r.edit_version AS version,
                  r.created_at::text AS created_at,
                  r.institution_id,
                  r.decision_reason
                FROM
                  institution_registration_requests r
                  JOIN users u ON u.id = r.user_id
                WHERE
                  r.user_id = $1
                ORDER BY
                  r.id DESC
                LIMIT
                  $2
                OFFSET
                  $3
            "#,
        )
        .bind(actor.user)
        .bind(i64::from(p.size))
        .bind(p.offset())
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }
    pub async fn institution_requests(
        &self,
        digest: &[u8],
        p: &PageRequest,
    ) -> AccountResult<Page<RequestRow>> {
        validate(p)?;
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        actor.need("manage_institutions")?;
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  institution_registration_requests
                WHERE
                  (
                    $1::text IS NULL
                    OR state = $1
                  )
                  AND (
                    $2 = ''
                    OR name ILIKE '%' || $2 || '%'
                  )
            "#,
        )
        .bind(&p.state)
        .bind(&p.query)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, RequestRow>(
            r#"
                SELECT
                  r.id,
                  r.user_id,
                  u.display_name AS user_name,
                  r.name,
                  r.description,
                  r.evidence,
                  r.state,
                  r.edit_version AS version,
                  r.created_at::text AS created_at,
                  r.institution_id,
                  r.decision_reason
                FROM
                  institution_registration_requests r
                  JOIN users u ON u.id = r.user_id
                WHERE
                  (
                    $1::text IS NULL
                    OR r.state = $1
                  )
                  AND (
                    $2 = ''
                    OR r.name ILIKE '%' || $2 || '%'
                  )
                ORDER BY
                  r.id
                LIMIT
                  $3
                OFFSET
                  $4
            "#,
        )
        .bind(&p.state)
        .bind(&p.query)
        .bind(i64::from(p.size))
        .bind(p.offset())
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }
    pub async fn decide_institution(
        &self,
        origin: AdminOrigin<'_>,
        id: i64,
        expected: i64,
        accept: bool,
        reason: &str,
    ) -> AccountResult<()> {
        guard::reason(reason)?;
        let (mut tx, actor) = self.begin(origin).await?;
        actor.need("manage_institutions")?;
        let old: Option<(i64, String, String, i64)> = sqlx::query_as(
            r#"
                SELECT
                  user_id,
                  name,
                  state,
                  edit_version
                FROM
                  institution_registration_requests
                WHERE
                  id = $1
                FOR UPDATE
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let (user, name, state, version) = old.ok_or(AccountError::NotFound)?;
        if state != "pending" {
            return Err(AccountError::AlreadyDecided);
        }
        if version != expected {
            return Err(AccountError::VersionConflict);
        }
        let institution = if accept {
            let active: bool = sqlx::query_scalar(
                r#"
                    SELECT
                      state = 'active'
                    FROM
                      users
                    WHERE
                      id = $1
                    FOR UPDATE
                "#,
            )
            .bind(user)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
            if !active {
                return Err(AccountError::AccountDisabled);
            }
            let institution: i64 = sqlx::query_scalar(
                r#"
                    INSERT INTO
                      institutions (name)
                    VALUES
                      ($1)
                    RETURNING
                      id
                "#,
            )
            .bind(&name)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
            sqlx::query(
                r#"
                    INSERT INTO
                      institution_memberships (user_id, institution_id, state)
                    VALUES
                      ($1, $2, 'active')
                "#,
            )
            .bind(user)
            .bind(institution)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
            sqlx::query(
                r#"
                    INSERT INTO
                      membership_permissions (user_id, institution_id, permission)
                    SELECT
                      $1,
                      $2,
                      p
                    FROM
                      unnest(
                        ARRAY[
                          'manage_members',
                          'manage_disciplines',
                          'create_materials',
                          'manage_materials',
                          'restrict_materials',
                          'review_restricted_materials'
                        ]
                      ) p
                "#,
            )
            .bind(user)
            .bind(institution)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
            Some(institution)
        } else {
            None
        };
        let state = if accept { "accepted" } else { "rejected" };
        sqlx::query(
            r#"
                UPDATE institution_registration_requests
                SET
                  state = $1,
                  institution_id = $2,
                  decided_at = clock_timestamp(),
                  edit_version = edit_version + 1,
                  decision_reason = $4
                WHERE
                  id = $3
            "#,
        )
        .bind(state)
        .bind(institution)
        .bind(id)
        .bind(reason.trim())
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        audit::write(&mut tx,&actor,AuditEvent {kind:"institution_request",id,institution,action:"decide_institution_registration",reason,
            before:json!({"state":"pending"}),after:json!({"state":state,"institutionId":institution,"firstManagerUserId":if accept {Some(user)} else {None}})})
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
