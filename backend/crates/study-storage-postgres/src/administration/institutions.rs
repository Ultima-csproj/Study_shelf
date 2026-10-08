use super::{
    Administration, Page, PageRequest,
    access::{self, AdminOrigin},
    audit::{self, AuditEvent},
    guard,
};
use crate::{account::session, error::map_sqlx_error};
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;
use study_core::account::{AccountError, AccountResult};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionRow {
    pub id: i64,
    pub name: String,
    pub version: i64,
    pub member_count: i64,
    pub manager_count: i64,
}
impl Administration {
    pub async fn institutions(
        &self,
        digest: &[u8],
        p: &PageRequest,
    ) -> AccountResult<Page<InstitutionRow>> {
        p.validate()?;
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        let all = actor.has("manage_institutions");
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
        if !all && scopes.is_empty() {
            return Err(AccountError::Forbidden);
        }
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  institutions
                WHERE
                  (
                    $1
                    OR id = ANY ($2)
                  )
                  AND (
                    $3 = ''
                    OR name ILIKE '%' || $3 || '%'
                    OR id::text = $3
                  )
            "#,
        )
        .bind(all)
        .bind(&scopes)
        .bind(&p.query)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, InstitutionRow>(
            r#"
                SELECT
                  i.id,
                  i.name,
                  i.edit_version AS version,
                  (
                    SELECT
                      count(*)
                    FROM
                      institution_memberships
                    WHERE
                      institution_id = i.id
                      AND state = 'active'
                  ) AS member_count,
                  (
                    SELECT
                      count(*)
                    FROM
                      institution_memberships m
                      JOIN users u ON u.id = m.user_id
                      JOIN membership_permissions p USING (user_id, institution_id)
                    WHERE
                      m.institution_id = i.id
                      AND m.state = 'active'
                      AND u.state = 'active'
                      AND p.permission = 'manage_members'
                  ) AS manager_count
                FROM
                  institutions i
                WHERE
                  (
                    $1
                    OR id = ANY ($2)
                  )
                  AND (
                    $3 = ''
                    OR i.name ILIKE '%' || $3 || '%'
                    OR i.id::text = $3
                  )
                ORDER BY
                  i.id
                LIMIT
                  $4
                OFFSET
                  $5
            "#,
        )
        .bind(all)
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
    pub async fn rename_institution(
        &self,
        origin: AdminOrigin<'_>,
        id: i64,
        expected: i64,
        name: &str,
        reason: &str,
    ) -> AccountResult<()> {
        guard::reason(reason)?;
        if name.trim().is_empty() || name.chars().count() > 200 {
            return Err(AccountError::Validation(
                "Название учреждения: 1–200 символов",
            ));
        }
        let (mut tx, actor) = self.begin(origin).await?;
        actor.need("manage_institutions")?;
        let old: Option<(String, i64)> = sqlx::query_as(
            r#"
                SELECT
                  name,
                  edit_version
                FROM
                  institutions
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
        sqlx::query(
            r#"
                UPDATE institutions
                SET
                  name = $1,
                  edit_version = edit_version + 1
                WHERE
                  id = $2
            "#,
        )
        .bind(name.trim())
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        audit::write(
            &mut tx,
            &actor,
            AuditEvent {
                kind: "institution",
                id,
                institution: Some(id),
                action: "rename_institution",
                reason,
                before: json!({"name":old}),
                after: json!({"name":name.trim()}),
            },
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn institution(&self, digest: &[u8], id: i64) -> AccountResult<InstitutionRow> {
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = access::read(&mut tx, digest).await?;
        actor.institution(&mut tx, id, false).await?;
        let item = sqlx::query_as::<_, InstitutionRow>(
            r#"
                SELECT
                  i.id,
                  i.name,
                  i.edit_version AS version,
                  (
                    SELECT
                      count(*)
                    FROM
                      institution_memberships
                    WHERE
                      institution_id = i.id
                      AND state = 'active'
                  ) AS member_count,
                  (
                    SELECT
                      count(*)
                    FROM
                      institution_memberships m
                      JOIN users u ON u.id = m.user_id
                      JOIN membership_permissions p USING (user_id, institution_id)
                    WHERE
                      m.institution_id = i.id
                      AND m.state = 'active'
                      AND u.state = 'active'
                      AND p.permission = 'manage_members'
                  ) AS manager_count
                FROM
                  institutions i
                WHERE
                  id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?
        .ok_or(AccountError::NotFound)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(item)
    }
}
