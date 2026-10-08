use super::super::{
    Administration, Page, PageRequest,
    access::{self, AdminOrigin},
    audit::{self, AuditEvent},
    guard,
};
use crate::{account::session, error::map_sqlx_error};

use serde_json::json;

use study_core::account::{AccountError, AccountResult};

use super::{RequestRow, validate};
impl Administration {
    pub async fn contributor_requests(
        &self,
        digest: &[u8],
        p: &PageRequest,
    ) -> AccountResult<Page<RequestRow>> {
        validate(p)?;
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        actor.need("manage_accounts")?;
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  contributor_link_requests r
                  JOIN users u ON u.id = r.user_id
                  JOIN contributors c ON c.id = r.contributor_id
                WHERE
                  (
                    $1::text IS NULL
                    OR r.state = $1
                  )
                  AND (
                    $2 = ''
                    OR u.display_name ILIKE '%' || $2 || '%'
                    OR c.display_name ILIKE '%' || $2 || '%'
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
                  r.contributor_id AS id,
                  r.user_id,
                  u.display_name AS user_name,
                  c.display_name AS name,
                  ''::text AS description,
                  CASE
                    WHEN $3 THEN r.evidence
                    ELSE NULL
                  END AS evidence,
                  r.state,
                  r.edit_version AS version,
                  r.created_at::text AS created_at,
                  NULL::bigint AS institution_id,
                  r.decision_reason
                FROM
                  contributor_link_requests r
                  JOIN users u ON u.id = r.user_id
                  JOIN contributors c ON c.id = r.contributor_id
                WHERE
                  (
                    $1::text IS NULL
                    OR r.state = $1
                  )
                  AND (
                    $2 = ''
                    OR u.display_name ILIKE '%' || $2 || '%'
                    OR c.display_name ILIKE '%' || $2 || '%'
                  )
                ORDER BY
                  r.created_at,
                  r.user_id,
                  r.contributor_id
                LIMIT
                  $4
                OFFSET
                  $5
            "#,
        )
        .bind(&p.state)
        .bind(&p.query)
        .bind(actor.has("view_account_private_data"))
        .bind(i64::from(p.size))
        .bind(p.offset())
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }
    pub async fn decide_contributor(
        &self,
        origin: AdminOrigin<'_>,
        user: i64,
        contributor: i64,
        expected: Option<i64>,
        accept: bool,
        reason: &str,
    ) -> AccountResult<()> {
        guard::reason(reason)?;
        let (mut tx, actor) = self.begin(origin).await?;
        actor.need("manage_accounts")?;
        if accept {
            actor.need("view_account_private_data")?;
        }
        let active: Option<bool> = sqlx::query_scalar(
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
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if active.is_none() {
            return Err(AccountError::NotFound);
        }
        if accept && active != Some(true) {
            return Err(AccountError::AccountDisabled);
        }
        let old: Option<(String, i64, String)> = sqlx::query_as(
            r#"
                SELECT
                  state,
                  edit_version,
                  evidence
                FROM
                  contributor_link_requests
                WHERE
                  user_id = $1
                  AND contributor_id = $2
                FOR UPDATE
            "#,
        )
        .bind(user)
        .bind(contributor)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let (state, version, evidence) = old.ok_or(AccountError::NotFound)?;
        if state != "pending" {
            return Err(AccountError::AlreadyDecided);
        }
        if expected.is_some_and(|v| v != version) {
            return Err(AccountError::VersionConflict);
        }
        if accept {
            if !actor.local && evidence.trim().chars().count() < 3 {
                return Err(AccountError::ContributorEvidenceMissing);
            }
            let changed = sqlx::query(
                r#"
                    UPDATE contributors
                    SET
                      user_id = $1
                    WHERE
                      id = $2
                      AND kind = 'person'
                      AND (
                        user_id IS NULL
                        OR user_id = $1
                      )
                "#,
            )
            .bind(user)
            .bind(contributor)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?
            .rows_affected();
            if changed != 1 {
                return Err(AccountError::ContributorAlreadyLinked);
            }
        }
        let state = if accept { "accepted" } else { "rejected" };
        sqlx::query(
            r#"
                UPDATE contributor_link_requests
                SET
                  state = $1,
                  decision_reason = $2,
                  edit_version = edit_version + 1
                WHERE
                  user_id = $3
                  AND contributor_id = $4
            "#,
        )
        .bind(state)
        .bind(reason.trim())
        .bind(user)
        .bind(contributor)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        audit::write(
            &mut tx,
            &actor,
            AuditEvent {
                kind: "contributor_request",
                id: contributor,
                institution: None,
                action: "decide_contributor_link",
                reason,
                before: json!({"state":"pending","userId":user}),
                after: json!({"state":state,"userId":user}),
            },
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
