use std::path::PathBuf;

use study_storage_sqlite::SqliteStorage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("study.sqlite3");

    let _storage = SqliteStorage::open(&db_path).await?;

    println!("SQLite готова: {}", db_path.display());
    Ok(())
}
