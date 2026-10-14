use super::{PostgresAccountRepository, session};
use crate::error::map_sqlx_error;
use sqlx::FromRow;
use study_core::account::{AccountError, AccountResult, ProfileInput};

#[derive(FromRow)]
pub struct Credential {
    pub user_id: i64,
    pub password_hash: String,
    pub password_version: i64,
}
impl PostgresAccountRepository {
    pub async fn register(
        &self,
        email: &str,
        hash: &str,
        profile: &ProfileInput,
    ) -> AccountResult<i64> {
        profile.validate()?;
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let name = profile.display_name();
        let id: i64 = sqlx::query_scalar(
            r#"
                INSERT INTO
                  users (display_name, state)
                VALUES
                  ($1, 'active')
                RETURNING
                  id
            "#,
        )
        .bind(&name)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let result = sqlx::query(
            r#"
                INSERT INTO
                  user_credentials (user_id, email, password_hash)
                VALUES
                  ($1, $2, $3)
            "#,
        )
        .bind(id)
        .bind(email)
        .bind(hash)
        .execute(&mut *tx)
        .await;
        match result {
            Err(sqlx::Error::Database(error))
                if error.constraint() == Some("user_credentials_email_key") =>
            {
                return Err(AccountError::EmailTaken);
            }
            Err(error) => return Err(map_sqlx_error(error).into()),
            Ok(_) => {}
        }
        let contributor: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO
              contributors (kind, display_name, user_id)
            VALUES
              ('person', $1, $2)
            RETURNING
              id
        "#,
        )
        .bind(&name)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        sqlx::query(
            r#"
            INSERT INTO
              user_profiles (
                user_id,
                first_name,
                last_name,
                primary_contributor_id
              )
            VALUES
              ($1, $2, $3, $4)
        "#,
        )
        .bind(id)
        .bind(profile.first_name.trim())
        .bind(profile.last_name.trim())
        .bind(contributor)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(id)
    }
    pub async fn credentials(&self, email: &str) -> AccountResult<Option<Credential>> {
        Ok(sqlx::query_as::<_, Credential>(
            r#"
                SELECT
                  c.user_id,
                  c.password_hash,
                  c.password_version
                FROM
                  user_credentials c
                  JOIN users u ON u.id = c.user_id
                WHERE
                  c.email = $1
                  AND u.state = 'active'
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?)
    }
    pub async fn own_credentials(&self, digest: &[u8]) -> AccountResult<Credential> {
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let value = sqlx::query_as::<_, Credential>(
            r#"
                SELECT
                  user_id,
                  password_hash,
                  password_version
                FROM
                  user_credentials
                WHERE
                  user_id = $1
            "#,
        )
        .bind(user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(value)
    }
    pub async fn change_password(
        &self,
        digest: &[u8],
        expected: i64,
        hash: &str,
    ) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest).await?.user_id;
        let result = sqlx::query(
            r#"
                UPDATE user_credentials
                SET
                  password_hash = $1,
                  password_version = password_version + 1
                WHERE
                  user_id = $2
                  AND password_version = $3
            "#,
        )
        .bind(hash)
        .bind(user)
        .bind(expected)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if result.rows_affected() != 1 {
            return Err(AccountError::VersionConflict);
        }
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
        .bind(user)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
