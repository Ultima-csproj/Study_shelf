mod collections;
mod disciplines;
use super::{SeedError, identities::World};
use sqlx::PgPool;

pub(super) async fn populate(pool: &PgPool, world: &World) -> Result<u64, SeedError> {
    disciplines::populate(pool, world).await?;
    collections::populate(pool, world).await
}
