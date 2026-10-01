use anyhow::Context;
use backend::api;
use backend::api::state::AppState;
use backend::catalog::CatalogService;
use backend::config::Config;
use study_storage_sqlite::SqliteStorage;
use study_storage_sqlite::catalog::SqliteCatalogRepository;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("backend=info,tower_http=debug"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = Config::from_env()?;

    let storage = SqliteStorage::open(&config.database_path).await?;

    let pool = storage.pool().clone();

    let repository = SqliteCatalogRepository::new(pool.clone());

    let service = CatalogService::new(repository);
    let state = AppState::new(service);
    let app = api::router(state);

    let listener = TcpListener::bind(config.bind_address)
        .await
        .context("Не удалось открыть адрес HTTP-сервера")?;

    tracing::info!(
        address = %listener.local_addr()?,
        database = %config.database_path.display(),
        "HTTP-сервер запущен"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    pool.close().await;

    Ok(())
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(
            error = %error,
            "Не удалось получить сигнал остановки"
        );
    }

    tracing::info!("Остановка HTTP-сервера");
}
