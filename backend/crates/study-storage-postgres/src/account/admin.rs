use super::PostgresAccountRepository;
use crate::error::map_sqlx_error;
use sqlx::PgConnection;
use study_core::account::{AccountError, AccountResult};

async fn active_user(conn: &mut PgConnection, user: i64) -> AccountResult<()> {
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
    .fetch_optional(conn)
    .await
    .map_err(map_sqlx_error)?;
    if active != Some(true) {
        return Err(AccountError::NotFound);
    }
    Ok(())
}
async fn audit(
    conn: &mut PgConnection,
    user: i64,
    action: &str,
    scope: Option<i64>,
) -> AccountResult<()> {
    sqlx::query(
        r#"
        INSERT INTO
          identity_audit (subject_user_id, action, scope_id, source)
        VALUES
          ($1, $2, $3, 'cli')
    "#,
    )
    .bind(user)
    .bind(action)
    .bind(scope)
    .execute(conn)
    .await
    .map_err(map_sqlx_error)?;
    Ok(())
}
impl PostgresAccountRepository {
    // These operations are deliberately exposed only through the local maintenance CLI.
    pub async fn platform_admin(&self, user: i64, grant: bool) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        active_user(&mut tx, user).await?;
        if grant {
            sqlx::query(
                r#"
                INSERT INTO
                  user_platform_permissions (user_id, permission)
                SELECT
                  $1,
                  p
                FROM
                  unnest(
                    ARRAY[
                      'moderate_public_materials',
                      'manage_accounts',
                      'manage_institutions',
                      'manage_platform_permissions'
                    ]
                  ) p
                ON CONFLICT DO NOTHING
            "#,
            )
            .bind(user)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        } else {
            sqlx::query(
                r#"
                DELETE FROM user_platform_permissions
                WHERE
                  user_id = $1
            "#,
            )
            .bind(user)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        }
        audit(
            &mut tx,
            user,
            if grant {
                "grant_platform_admin"
            } else {
                "revoke_platform_admin"
            },
            None,
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn institution_admin(
        &self,
        user: i64,
        institution: i64,
        grant: bool,
    ) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        active_user(&mut tx, user).await?;
        let exists: bool = sqlx::query_scalar(
            r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      institutions
                    WHERE
                      id = $1
                  )
            "#,
        )
        .bind(institution)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if !exists {
            return Err(AccountError::NotFound);
        }
        if grant {
            sqlx::query(
                r#"
                    INSERT INTO
                      institution_memberships (user_id, institution_id, state)
                    VALUES
                      ($1, $2, 'active')
                    ON CONFLICT (user_id, institution_id) DO UPDATE
                    SET
                      state = 'active'
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
                      'manage_materials'
                    ]
                  ) p
                ON CONFLICT DO NOTHING
            "#,
            )
            .bind(user)
            .bind(institution)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        } else {
            sqlx::query(
                r#"
                    DELETE FROM membership_permissions
                    WHERE
                      user_id = $1
                      AND institution_id = $2
                "#,
            )
            .bind(user)
            .bind(institution)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        }
        audit(
            &mut tx,
            user,
            if grant {
                "grant_institution_admin"
            } else {
                "revoke_institution_admin"
            },
            Some(institution),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn decide_membership(
        &self,
        user: i64,
        institution: i64,
        role: &str,
        accept: bool,
    ) -> AccountResult<()> {
        self.decide_membership_as(None, user, institution, role, accept)
            .await
    }
    pub async fn review_membership(
        &self,
        digest: &[u8],
        user: i64,
        institution: i64,
        role: &str,
        accept: bool,
    ) -> AccountResult<()> {
        self.decide_membership_as(Some(digest), user, institution, role, accept)
            .await
    }
    async fn decide_membership_as(
        &self,
        digest: Option<&[u8]>,
        user: i64,
        institution: i64,
        role: &str,
        accept: bool,
    ) -> AccountResult<()> {
        if !matches!(role, "student" | "teacher") {
            return Err(AccountError::Validation(
                "Роль должна быть student или teacher",
            ));
        }
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let actor = if let Some(digest) = digest {
            let actor = super::session::require(&mut tx, digest).await?.user_id;
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
            .bind(actor)
            .bind(institution)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
            if !allowed {
                return Err(AccountError::Forbidden);
            }
            Some(actor)
        } else {
            None
        };
        active_user(&mut tx, user).await?;
        let result = sqlx::query(
            r#"
            UPDATE institution_memberships
            SET
              state = $1
            WHERE
              user_id = $2
              AND institution_id = $3
              AND state = 'pending'
        "#,
        )
        .bind(if accept { "active" } else { "rejected" })
        .bind(user)
        .bind(institution)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if result.rows_affected() != 1 {
            return Err(AccountError::NotFound);
        }
        // A previously revoked membership must not recover its old grants through a fresh application.
        sqlx::query(
            r#"
            DELETE FROM membership_permissions
            WHERE
              user_id = $1
              AND institution_id = $2
        "#,
        )
        .bind(user)
        .bind(institution)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        sqlx::query(
            r#"
            DELETE FROM membership_academic_roles
            WHERE
              user_id = $1
              AND institution_id = $2
        "#,
        )
        .bind(user)
        .bind(institution)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if accept {
            sqlx::query(
                r#"
                INSERT INTO
                  membership_academic_roles (user_id, institution_id, role)
                VALUES
                  ($1, $2, $3)
            "#,
            )
            .bind(user)
            .bind(institution)
            .bind(role)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        }
        decision_audit(
            &mut tx,
            actor,
            user,
            if accept {
                "accept_membership"
            } else {
                "reject_membership"
            },
            Some(institution),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn link_contributor(
        &self,
        user: i64,
        contributor: i64,
        accept: bool,
    ) -> AccountResult<()> {
        self.link_contributor_as(None, user, contributor, accept)
            .await
    }
    pub async fn review_contributor(
        &self,
        digest: &[u8],
        user: i64,
        contributor: i64,
        accept: bool,
    ) -> AccountResult<()> {
        self.link_contributor_as(Some(digest), user, contributor, accept)
            .await
    }
    async fn link_contributor_as(
        &self,
        digest: Option<&[u8]>,
        user: i64,
        contributor: i64,
        accept: bool,
    ) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let actor = if let Some(digest) = digest {
            let actor = super::session::require(&mut tx, digest).await?.user_id;
            let allowed: bool = sqlx::query_scalar(
                r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      user_platform_permissions
                    WHERE
                      user_id = $1
                      AND permission = 'manage_accounts'
                  )
            "#,
            )
            .bind(actor)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
            if !allowed {
                return Err(AccountError::Forbidden);
            }
            Some(actor)
        } else {
            None
        };
        active_user(&mut tx, user).await?;
        let pending: bool = sqlx::query_scalar(
            r#"
            SELECT
              EXISTS (
                SELECT
                  1
                FROM
                  contributor_link_requests
                WHERE
                  user_id = $1
                  AND contributor_id = $2
                  AND state = 'pending'
              )
        "#,
        )
        .bind(user)
        .bind(contributor)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if !pending {
            return Err(AccountError::NotFound);
        }
        if accept {
            let result = sqlx::query(
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
            .map_err(map_sqlx_error)?;
            if result.rows_affected() != 1 {
                return Err(AccountError::ContributorAlreadyLinked);
            }
        }
        sqlx::query(
            r#"
                UPDATE contributor_link_requests
                SET
                  state = $1
                WHERE
                  user_id = $2
                  AND contributor_id = $3
            "#,
        )
        .bind(if accept { "accepted" } else { "rejected" })
        .bind(user)
        .bind(contributor)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        decision_audit(
            &mut tx,
            actor,
            user,
            if accept {
                "accept_contributor"
            } else {
                "reject_contributor"
            },
            Some(contributor),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}

async fn decision_audit(
    conn: &mut PgConnection,
    actor: Option<i64>,
    user: i64,
    action: &str,
    scope: Option<i64>,
) -> AccountResult<()> {
    sqlx::query(
        r#"
        INSERT INTO
          identity_audit (
            actor_user_id,
            subject_user_id,
            action,
            scope_id,
            source
          )
        VALUES
          ($1, $2, $3, $4, $5)
    "#,
    )
    .bind(actor)
    .bind(user)
    .bind(action)
    .bind(scope)
    .bind(if actor.is_some() { "http" } else { "cli" })
    .execute(conn)
    .await
    .map_err(map_sqlx_error)?;
    Ok(())
}
