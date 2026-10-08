use super::Administration;
use crate::{account::session, error::map_sqlx_error};
use serde::Serialize;
use sqlx::{FromRow, PgConnection};
use study_core::account::{AccountError, AccountResult};

pub enum AdminOrigin<'a> {
    Session(&'a [u8]),
    LocalMaintenance,
}
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ManagedInstitution {
    pub id: i64,
    pub name: String,
    pub permissions: Vec<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccess {
    pub permissions: Vec<String>,
    pub institutions: Vec<ManagedInstitution>,
}
pub(crate) struct Access {
    pub user: Option<i64>,
    pub permissions: Vec<String>,
    pub local: bool,
}
impl Access {
    pub fn has(&self, permission: &str) -> bool {
        self.local || self.permissions.iter().any(|p| p == permission)
    }
    pub fn need(&self, permission: &str) -> AccountResult<()> {
        if !self.has(permission) {
            return Err(AccountError::Forbidden);
        }
        Ok(())
    }
    pub async fn institution(
        &self,
        conn: &mut PgConnection,
        id: i64,
        write: bool,
    ) -> AccountResult<()> {
        if self.local || (!write && self.has("manage_institutions")) {
            return Ok(());
        }
        let allowed: bool = sqlx::query_scalar(
            r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      institution_memberships m
                      JOIN membership_permissions p USING (user_id, institution_id)
                    WHERE
                      m.user_id = $1
                      AND m.institution_id = $2
                      AND m.state = 'active'
                      AND p.permission = 'manage_members'
                  )
            "#,
        )
        .bind(self.user)
        .bind(id)
        .fetch_one(conn)
        .await
        .map_err(map_sqlx_error)?;
        if !allowed {
            return Err(AccountError::Forbidden);
        }
        Ok(())
    }
}
pub(crate) async fn read(conn: &mut PgConnection, digest: &[u8]) -> AccountResult<Access> {
    let user = session::read(conn, digest).await?.user_id;
    load(conn, Some(user)).await
}
pub(crate) async fn load(conn: &mut PgConnection, user: Option<i64>) -> AccountResult<Access> {
    let permissions = if let Some(user) = user {
        sqlx::query_scalar(
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
        .bind(user)
        .fetch_all(conn)
        .await
        .map_err(map_sqlx_error)?
    } else {
        Vec::new()
    };
    Ok(Access {
        user,
        permissions,
        local: user.is_none(),
    })
}
impl Administration {
    pub async fn access(&self, digest: &[u8]) -> AccountResult<AdminAccess> {
        let mut tx = session::snapshot(&self.pool).await?;
        let actor = read(&mut tx, digest).await?;
        let institutions = sqlx::query_as::<_, ManagedInstitution>(
            r#"
                SELECT
                  i.id,
                  i.name,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      membership_permissions
                    WHERE
                      user_id = m.user_id
                      AND institution_id = i.id
                    ORDER BY
                      permission
                  ) AS permissions
                FROM
                  institutions i
                  JOIN institution_memberships m ON m.institution_id = i.id
                WHERE
                  m.user_id = $1
                  AND m.state = 'active'
                  AND EXISTS (
                    SELECT
                      1
                    FROM
                      membership_permissions p
                    WHERE
                      p.user_id = m.user_id
                      AND p.institution_id = i.id
                      AND p.permission IN (
                          'manage_members',
                          'restrict_materials',
                          'review_restricted_materials'
                      )
                  )
                ORDER BY
                  i.id
            "#,
        )
        .bind(actor.user)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(AdminAccess {
            permissions: actor.permissions,
            institutions,
        })
    }
}
