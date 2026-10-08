use super::{Administration, access};
use crate::{account::session, error::map_sqlx_error};
use serde::Serialize;
use sqlx::FromRow;
use study_core::account::{AccountError, AccountResult};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    pub accounts: Option<i64>,
    pub institutions: Option<i64>,
    pub pending_members: i64,
    pub pending_contributors: i64,
    pub pending_institutions: i64,
    pub pending_materials: Option<i64>,
    #[sqlx(skip)]
    pub member_attention: Vec<MemberAttention>,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MemberAttention {
    pub institution_id: i64,
    pub institution_name: String,
    pub pending_count: i64,
}
impl Administration {
    pub async fn overview(&self, digest: &[u8]) -> AccountResult<Overview> {
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
        if scopes.is_empty()
            && !actor
                .permissions
                .iter()
                .any(|p| p != "view_account_private_data")
        {
            let material_staff: bool = sqlx::query_scalar(include_str!(
                "../material/workspace/sql/restriction_queue_access.sql"
            ))
            .bind(actor.user)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
            if !material_staff {
                return Err(AccountError::Forbidden);
            }
        }
        let mut item = sqlx::query_as::<_, Overview>(
            r#"
                SELECT
                  CASE
                    WHEN $1 THEN (
                      SELECT
                        count(*)
                      FROM
                        users
                    )
                    ELSE NULL
                  END AS accounts,
                  CASE
                    WHEN $2 THEN (
                      SELECT
                        count(*)
                      FROM
                        institutions
                    )
                    WHEN cardinality($4::bigint[]) > 0 THEN cardinality($4::bigint[])::bigint
                    ELSE NULL
                  END AS institutions,
                  (
                    SELECT
                      count(*)
                    FROM
                      institution_memberships
                    WHERE
                      state = 'pending'
                      AND (
                        $2
                        OR institution_id = ANY ($4)
                      )
                  ) AS pending_members,
                  CASE
                    WHEN $3 THEN (
                      SELECT
                        count(*)
                      FROM
                        contributor_link_requests
                      WHERE
                        state = 'pending'
                    )
                    ELSE 0
                  END AS pending_contributors,
                  CASE
                    WHEN $2 THEN (
                      SELECT
                        count(*)
                      FROM
                        institution_registration_requests
                      WHERE
                        state = 'pending'
                    )
                    ELSE 0
                  END AS pending_institutions,
                  CASE
                    WHEN $5 THEN (
                      SELECT
                        count(*)
                      FROM material_revisions AS revision
                      JOIN materials AS material
                        ON material.id = revision.material_id
                      WHERE revision.state = 'pending_moderation'
                        AND material.lifecycle = 'active'
                    )
                    ELSE NULL
                  END AS pending_materials
            "#,
        )
        .bind(actor.has("manage_accounts") || actor.has("manage_platform_permissions"))
        .bind(actor.has("manage_institutions"))
        .bind(actor.has("manage_accounts"))
        .bind(&scopes)
        .bind(actor.has("moderate_public_materials"))
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

        item.member_attention = sqlx::query_as(include_str!("sql/member_attention.sql"))
            .bind(actor.has("manage_institutions"))
            .bind(&scopes)
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(item)
    }
}
