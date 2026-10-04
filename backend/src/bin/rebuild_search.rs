use backend::config::Config;
use study_storage_postgres::{PostgresStorage, catalog::PostgresCatalogRepository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let storage = PostgresStorage::open(
        &config.database_url,
        config.max_connections,
        config.acquire_timeout,
    )
    .await?;
    let count = PostgresCatalogRepository::new(storage.pool().clone())
        .rebuild_search()
        .await?;
    println!("Rebuilt search projection for {count} published materials");
    storage.pool().close().await;
    Ok(())
}
