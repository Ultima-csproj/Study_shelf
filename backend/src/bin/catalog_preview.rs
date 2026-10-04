use backend::{catalog::CatalogService, config::Config};
use study_core::catalog::{CatalogQuery, CatalogSort};
use study_storage_postgres::{PostgresStorage, catalog::PostgresCatalogRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let storage = PostgresStorage::open(
        &config.database_url,
        config.max_connections,
        config.acquire_timeout,
    )
    .await?;
    let service = CatalogService::new(PostgresCatalogRepository::new(storage.pool().clone()));
    let query = CatalogQuery {
        text: std::env::args().nth(1).unwrap_or_default(),
        material_types: vec![],
        discipline_ids: vec![],
        teacher_ids: vec![],
        courses: vec![],
        years: vec![],
        tags: vec![],
        sort: CatalogSort::Newest,
        page: 1,
        page_size: 5,
    };
    let page = service.list(query).await?;
    println!("Public materials: {}, page {}", page.total, page.page);
    for item in page.items {
        println!("{} — {}", item.id.0, item.title);
    }
    storage.pool().close().await;
    Ok(())
}
