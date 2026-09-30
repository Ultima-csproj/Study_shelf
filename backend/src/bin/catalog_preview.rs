use std::path::PathBuf;

use backend::catalog::CatalogService;
use study_core::catalog::{CatalogQuery, CatalogSort};
use study_storage_sqlite::SqliteStorage;
use study_storage_sqlite::catalog::SqliteCatalogRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("study.sqlite3");
    let storage = SqliteStorage::open(db_path).await?;
    let service = CatalogService::new(SqliteCatalogRepository::new(storage.pool().clone()));
    let query = CatalogQuery {
        text: std::env::args().skip(1).collect::<Vec<_>>().join(" "),
        material_types: Vec::new(),
        discipline_ids: Vec::new(),
        teacher_ids: Vec::new(),
        courses: Vec::new(),
        years: Vec::new(),
        tags: Vec::new(),
        sort: CatalogSort::Newest,
        page: 1,
        page_size: 12,
    };
    let view = service.list(query).await?;
    println!(
        "Найдено: {}. Страница {}/{}",
        view.total, view.page, view.page_count
    );
    for item in view.items {
        println!(
            "{} · {} · {}",
            item.material.id.0, item.material.title, item.discipline.name
        );
    }
    Ok(())
}
