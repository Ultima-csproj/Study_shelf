use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool, Postgres, Transaction};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenError {
    #[error("invalid PostgreSQL connection options")]
    Options(#[source] sqlx::Error),
    #[error("cannot connect to PostgreSQL")]
    Connect(#[source] sqlx::Error),
    #[error("PostgreSQL schema migration failed")]
    Migration(#[source] sqlx::migrate::MigrateError),
}

pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub async fn open(
        url: &str,
        max_connections: u32,
        timeout: Duration,
    ) -> Result<Self, OpenError> {
        let options: PgConnectOptions = url.parse().map_err(OpenError::Options)?;
        let options = options.disable_statement_logging();
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(timeout)
            .after_connect(|connection, _| {
                Box::pin(async move {
                    sqlx::query("SET TIME ZONE 'UTC'")
                        .execute(&mut *connection)
                        .await?;
                    sqlx::query("SET statement_timeout = '15s'")
                        .execute(&mut *connection)
                        .await?;
                    sqlx::query("SET lock_timeout = '5s'")
                        .execute(&mut *connection)
                        .await?;
                    Ok(())
                })
            })
            .connect_with(options)
            .await
            .map_err(OpenError::Connect)?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(OpenError::Migration)?;
        Ok(Self { pool })
    }
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

pub(crate) async fn read_snapshot(
    pool: &PgPool,
) -> study_core::error::RepositoryResult<Transaction<'_, Postgres>> {
    let pool = pool.clone();
    // Finish BEGIN even when an HTTP reader is cancelled. If the caller disappears,
    // the completed task drops its Transaction and sqlx rolls it back before reuse.
    let opening = tokio::spawn(async move {
        pool.begin_with(
            r#"
                BEGIN TRANSACTION
                    ISOLATION LEVEL REPEATABLE READ
                    READ ONLY
            "#,
        )
        .await
    });
    opening
        .await
        .map_err(study_core::error::RepositoryError::unexpected)?
        .map_err(crate::error::map_sqlx_error)
}
