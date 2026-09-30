use std::path::PathBuf;

use study_storage_sqlite::SqliteStorage;

#[path = "../dev_seed/mod.rs"]
mod dev_seed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("study.sqlite3");
    let storage = SqliteStorage::open(&db_path).await?;
    let inserted = dev_seed::populate(storage.pool()).await?;
    println!(
        "Добавлено {inserted} демонстрационных материалов: {}",
        db_path.display()
    );
    Ok(())
}
