use super::{
    Administration,
    access::{self, Access, AdminOrigin},
};
use crate::{account::session, error::map_sqlx_error};
use sqlx::{PgConnection, Postgres, Transaction};
use study_core::account::{AccountError, AccountResult};

impl Administration {
    pub(crate) async fn begin(
        &self,
        origin: AdminOrigin<'_>,
    ) -> AccountResult<(Transaction<'_, Postgres>, Access)> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        // A single transaction advisory lock establishes the ordering before actor/subject row locks.
        sqlx::query(
            r#"
                SELECT
                  pg_advisory_xact_lock(734219013)
            "#,
        )
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let user = match origin {
            AdminOrigin::Session(digest) => Some(session::require(&mut tx, digest).await?.user_id),
            AdminOrigin::LocalMaintenance => None,
        };
        let actor = access::load(&mut tx, user).await?;
        Ok((tx, actor))
    }
}
pub(crate) struct LastManagers {
    platform: bool,
    institutions: Vec<i64>,
}
impl LastManagers {
    pub async fn capture(conn: &mut PgConnection, user: i64) -> AccountResult<Self> {
        let platform = sqlx::query_scalar(
            r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      users u
                      JOIN user_platform_permissions p ON p.user_id = u.id
                    WHERE
                      u.id = $1
                      AND u.state = 'active'
                      AND p.permission = 'manage_platform_permissions'
                  )
            "#,
        )
        .bind(user)
        .fetch_one(&mut *conn)
        .await
        .map_err(map_sqlx_error)?;
        let institutions = sqlx::query_scalar(
            r#"
                SELECT
                  m.institution_id
                FROM
                  institution_memberships m
                  JOIN users u ON u.id = m.user_id
                  JOIN membership_permissions p USING (user_id, institution_id)
                WHERE
                  m.user_id = $1
                  AND u.state = 'active'
                  AND m.state = 'active'
                  AND p.permission = 'manage_members'
                ORDER BY
                  m.institution_id
            "#,
        )
        .bind(user)
        .fetch_all(&mut *conn)
        .await
        .map_err(map_sqlx_error)?;
        Ok(Self {
            platform,
            institutions,
        })
    }
    pub async fn check(&self, conn: &mut PgConnection) -> AccountResult<()> {
        if self.platform {
            let exists: bool = sqlx::query_scalar(
                r#"
                    SELECT
                      EXISTS (
                        SELECT
                          1
                        FROM
                          users u
                          JOIN user_platform_permissions p ON p.user_id = u.id
                        WHERE
                          u.state = 'active'
                          AND p.permission = 'manage_platform_permissions'
                      )
                "#,
            )
            .fetch_one(&mut *conn)
            .await
            .map_err(map_sqlx_error)?;
            if !exists {
                return Err(AccountError::LastPlatformAdministrator);
            }
        }
        let missing: bool = sqlx::query_scalar(
            r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      unnest($1::bigint[]) x (id)
                    WHERE
                      NOT EXISTS (
                        SELECT
                          1
                        FROM
                          institution_memberships m
                          JOIN users u ON u.id = m.user_id
                          JOIN membership_permissions p USING (user_id, institution_id)
                        WHERE
                          m.institution_id = x.id
                          AND u.state = 'active'
                          AND m.state = 'active'
                          AND p.permission = 'manage_members'
                      )
                  )
            "#,
        )
        .bind(&self.institutions)
        .fetch_one(conn)
        .await
        .map_err(map_sqlx_error)?;
        if missing {
            return Err(AccountError::LastInstitutionManager);
        }
        Ok(())
    }
}
pub(crate) fn reason(value: &str) -> AccountResult<()> {
    if value.trim().chars().count() < 3 || value.chars().count() > 2000 {
        return Err(AccountError::Validation("Причина: от 3 до 2000 символов"));
    }
    Ok(())
}
