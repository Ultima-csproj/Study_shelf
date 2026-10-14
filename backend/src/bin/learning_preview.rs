use backend::config::Config;
use study_core::collection::CollectionRepository;
use study_core::discipline::DisciplineRepository;
use study_core::learning::{LearningQuery, LearningSort, PageRequest};
use study_storage_postgres::{
    PostgresStorage, collection::PostgresCollectionRepository,
    discipline::PostgresDisciplineRepository,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let storage = PostgresStorage::open(
        &config.database_url,
        config.max_connections,
        config.acquire_timeout,
    )
    .await?;
    let disciplines = PostgresDisciplineRepository::new(storage.pool().clone());
    let collections = PostgresCollectionRepository::new(storage.pool().clone());
    let page = disciplines.list(&LearningQuery::default()).await?;
    println!("Дисциплин: {}", page.total);
    if let Some(first) = page.items.first()
        && let Some(detail) = disciplines.get_by_id(first.id).await?
    {
        println!(
            "{}: {} материалов, {} шагов, {} видимых разделов",
            detail.summary.name,
            detail.summary.material_count,
            detail.summary.step_count,
            detail.sections.len()
        );
        if let Some(section) = detail.sections.first() {
            let page = disciplines
                .section_items(
                    first.id,
                    section.id,
                    PageRequest {
                        page: 2,
                        page_size: 10,
                    },
                )
                .await?;
            if let Some(page) = page {
                println!(
                    "Следующая страница раздела: {} пунктов",
                    page.items.items.len()
                );
            }
        }
    }
    let query = LearningQuery {
        sort: LearningSort::Newest,
        ..LearningQuery::default()
    };
    let page = collections.list(&query).await?;
    println!("Публичных подборок: {}", page.page.total);
    if let Some(first) = page.page.items.first()
        && let Some(detail) = collections.get_by_id(first.id).await?
    {
        println!(
            "{}: {} шагов, {} уникальных материалов, {} видимых разделов",
            detail.summary.title,
            detail.summary.step_count,
            detail.summary.unique_material_count,
            detail.sections.len()
        );
        if let Some(section) = detail.sections.first() {
            let page = collections
                .section_items(
                    first.id,
                    section.id,
                    PageRequest {
                        page: 2,
                        page_size: 10,
                    },
                )
                .await?;
            if let Some(page) = page {
                println!(
                    "Следующая страница маршрута: {} пунктов",
                    page.items.items.len()
                );
            }
        }
    }
    storage.pool().close().await;
    Ok(())
}
