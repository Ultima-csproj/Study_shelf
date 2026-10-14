use super::{
    Administration, Page, PageRequest,
    access::{self, Access},
};
use crate::{account::session, error::map_sqlx_error};
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, PgConnection};
use std::sync::LazyLock;
use study_core::account::{AccountError, AccountResult};
const FILTER: &str = r#"
    (
      $1
      OR (
        $2
        AND a.target_kind IN ('user', 'contributor_request')
      )
      OR (
        $3
        AND a.target_kind IN ('institution', 'institution_request', 'membership')
      )
      OR a.institution_id = ANY($4)
    )
    AND (
      $5 = ''
      OR a.action ILIKE '%' || $5 || '%'
    )
"#;
static COUNT: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
            SELECT
              count(*)
            FROM
              administration_audit a
            WHERE
              {FILTER}
        "#
    )
});
static SELECT: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
            SELECT
              a.id,
              a.actor_user_id,
              u.display_name AS actor_name,
              a.target_kind,
              a.target_id,
              a.institution_id,
              a.action,
              a.reason,
              a.before_data,
              a.after_data,
              a.source,
              a.created_at::text AS created_at
            FROM
              administration_audit a
              LEFT JOIN users u ON u.id = a.actor_user_id
            WHERE
              {FILTER}
            ORDER BY
              a.id DESC
            LIMIT
              $6
            OFFSET
              $7
        "#
    )
});

pub(crate) struct AuditEvent<'a> {
    pub kind: &'static str,
    pub id: i64,
    pub institution: Option<i64>,
    pub action: &'static str,
    pub reason: &'a str,
    pub before: Value,
    pub after: Value,
}
pub(crate) async fn write(
    conn: &mut PgConnection,
    actor: &Access,
    event: AuditEvent<'_>,
) -> AccountResult<()> {
    sqlx::query(
        r#"
            INSERT INTO
              administration_audit (
                actor_user_id,
                target_kind,
                target_id,
                institution_id,
                action,
                reason,
                before_data,
                after_data,
                source
              )
            VALUES
              ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(actor.user)
    .bind(event.kind)
    .bind(event.id)
    .bind(event.institution)
    .bind(event.action)
    .bind(event.reason.trim())
    .bind(event.before)
    .bind(event.after)
    .bind(if actor.local { "cli" } else { "http" })
    .execute(conn)
    .await
    .map_err(map_sqlx_error)?;
    Ok(())
}
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AuditRow {
    pub id: i64,
    pub actor_user_id: Option<i64>,
    pub actor_name: Option<String>,
    pub target_kind: String,
    pub target_id: i64,
    pub institution_id: Option<i64>,
    pub action: String,
    pub reason: String,
    pub before_data: Value,
    pub after_data: Value,
    pub source: String,
    pub created_at: String,
}
impl Administration {
    pub async fn audit(&self, digest: &[u8], p: &PageRequest) -> AccountResult<Page<AuditRow>> {
        p.validate()?;
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        let scopes: Vec<i64> = sqlx::query_scalar(
            r#"
                SELECT
                  m.institution_id
                FROM
                  institution_memberships m
                  JOIN membership_permissions p USING (user_id, institution_id)
                WHERE
                  m.user_id = $1
                  AND m.state = 'active'
                  AND p.permission = 'manage_members'
            "#,
        )
        .bind(actor.user)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let all = actor.has("manage_platform_permissions");
        let accounts = actor.has("manage_accounts");
        let institutions = actor.has("manage_institutions");
        if !all && !accounts && !institutions && scopes.is_empty() {
            return Err(AccountError::Forbidden);
        }
        let total = sqlx::query_scalar(&**COUNT)
            .bind(all)
            .bind(accounts)
            .bind(institutions)
            .bind(&scopes)
            .bind(&p.query)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, AuditRow>(&**SELECT)
            .bind(all)
            .bind(accounts)
            .bind(institutions)
            .bind(&scopes)
            .bind(&p.query)
            .bind(i64::from(p.size))
            .bind(p.offset())
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }
}
