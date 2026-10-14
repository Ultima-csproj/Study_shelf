use super::PostgresAccountRepository;
use crate::error::map_sqlx_error;
use sqlx::{FromRow, PgConnection};
use study_core::account::{AccountError, AccountResult};
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct Session {
    pub user_id: i64,
    pub csrf_hash: Vec<u8>,
}
// Writers lock the user before any session/credential row to keep a consistent lock order.
pub(crate) async fn require(conn: &mut PgConnection, digest: &[u8]) -> AccountResult<Session> {
    let user: Option<i64> = sqlx::query_scalar(
        r#"
            SELECT
              user_id
            FROM
              user_sessions
            WHERE
              token_hash = $1
        "#,
    )
    .bind(digest)
    .fetch_optional(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    let user = user.ok_or(AccountError::SessionExpired)?;
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
    .fetch_optional(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    if active != Some(true) {
        return Err(AccountError::SessionExpired);
    }
    sqlx::query_as::<_, Session>(
        r#"
            SELECT
              s.user_id,
              s.csrf_hash
            FROM
              user_sessions s
              JOIN users u ON u.id = s.user_id
            WHERE
              s.token_hash = $1
              AND s.revoked_at IS NULL
              AND s.expires_at > clock_timestamp()
              AND u.state = 'active'
            FOR SHARE OF
              s
        "#,
    )
    .bind(digest)
    .fetch_optional(conn)
    .await
    .map_err(map_sqlx_error)?
    .ok_or(AccountError::SessionExpired)
}
pub(crate) async fn snapshot(
    pool: &sqlx::PgPool,
) -> AccountResult<sqlx::Transaction<'_, sqlx::Postgres>> {
    Ok(crate::connection::read_snapshot(pool).await?)
}
pub(crate) async fn read(conn: &mut PgConnection, digest: &[u8]) -> AccountResult<Session> {
    sqlx::query_as::<_, Session>(
        r#"
            SELECT
              s.user_id,
              s.csrf_hash
            FROM
              user_sessions s
              JOIN users u ON u.id = s.user_id
            WHERE
              s.token_hash = $1
              AND s.revoked_at IS NULL
              AND s.expires_at > clock_timestamp()
              AND u.state = 'active'
        "#,
    )
    .bind(digest)
    .fetch_optional(conn)
    .await
    .map_err(map_sqlx_error)?
    .ok_or(AccountError::SessionExpired)
}
impl PostgresAccountRepository {
    pub async fn session(&self, digest: &[u8]) -> AccountResult<Option<Session>> {
        Ok(sqlx::query_as::<_, Session>(
            r#"
                SELECT
                  s.user_id,
                  s.csrf_hash
                FROM
                  user_sessions s
                  JOIN users u ON u.id = s.user_id
                WHERE
                  s.token_hash = $1
                  AND s.revoked_at IS NULL
                  AND s.expires_at > clock_timestamp()
                  AND u.state = 'active'
            "#,
        )
        .bind(digest)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?)
    }
    pub async fn create_session(
        &self,
        user: i64,
        version: i64,
        digest: &[u8],
        csrf: &[u8],
        expires: OffsetDateTime,
    ) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let active: Option<bool> = sqlx::query_scalar(
            r#"
                SELECT
                  state = 'active'
                FROM
                  users
                WHERE
                  id = $1
                FOR SHARE
            "#,
        )
        .bind(user)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if active != Some(true) {
            return Err(AccountError::InvalidCredentials);
        }
        let current: Option<i64> = sqlx::query_scalar(
            r#"
                SELECT
                  c.password_version
                FROM
                  user_credentials c
                  JOIN users u ON u.id = c.user_id
                WHERE
                  c.user_id = $1
                  AND u.state = 'active'
                FOR SHARE OF
                  c
            "#,
        )
        .bind(user)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if current != Some(version) {
            return Err(AccountError::InvalidCredentials);
        }
        sqlx::query(
            r#"
            INSERT INTO
              user_sessions (token_hash, user_id, csrf_hash, expires_at)
            VALUES
              ($1, $2, $3, $4)
        "#,
        )
        .bind(digest)
        .bind(user)
        .bind(csrf)
        .bind(expires)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn logout(&self, digest: &[u8]) -> AccountResult<()> {
        sqlx::query(
            r#"
            UPDATE user_sessions
            SET
              revoked_at = clock_timestamp()
            WHERE
              token_hash = $1
              AND revoked_at IS NULL
        "#,
        )
        .bind(digest)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn logout_others(&self, digest: &[u8]) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let session = require(&mut tx, digest).await?;
        sqlx::query(
            r#"
            UPDATE user_sessions
            SET
              revoked_at = clock_timestamp()
            WHERE
              user_id = $1
              AND token_hash <> $2
              AND revoked_at IS NULL
        "#,
        )
        .bind(session.user_id)
        .bind(digest)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn cleanup_sessions(&self) -> AccountResult<u64> {
        Ok(sqlx::query(
            r#"
            DELETE FROM user_sessions
            WHERE
              expires_at < now() - interval '7 days'
              OR revoked_at < now() - interval '7 days'
        "#,
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?
        .rows_affected())
    }
}
