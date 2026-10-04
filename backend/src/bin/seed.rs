use backend::config::Config;
use study_storage_postgres::PostgresStorage;
#[path = "../dev_seed/mod.rs"]
mod dev_seed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let storage = PostgresStorage::open(
        &config.database_url,
        config.max_connections,
        config.acquire_timeout,
    )
    .await?;
    let inserted = dev_seed::populate(storage.pool()).await?;
    println!("Добавлено {inserted} демонстрационных материалов в PostgreSQL");
    storage.pool().close().await;
    Ok(())
}
