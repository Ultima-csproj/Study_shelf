mod data;
mod identities;
mod materials;
use sqlx::PgPool;

#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    #[error("development seed database operation failed")]
    Database(#[from] sqlx::Error),
    #[error("development material fixture failed")]
    Material(#[from] study_storage_postgres::material::MaterialWriteError),
    #[error("invalid fixture timestamp")]
    Timestamp(#[from] study_core::timestamp::TimestampError),
    #[error("invalid fixture date")]
    Date(#[from] time::error::Parse),
}
pub async fn populate(pool: &PgPool) -> Result<u64, SeedError> {
    let world = identities::populate(pool).await?;
    materials::populate(pool, world).await
}
