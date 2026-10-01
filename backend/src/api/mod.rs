mod catalog;
pub mod error;
pub mod state;

use axum::{Json, Router, routing::get};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use self::error::ApiError;
use self::state::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/catalog", get(catalog::list))
        .route("/api/materials/{id}", get(catalog::get_by_id))
        .fallback(|| async { ApiError::not_found("Маршрут не найден") })
        .method_not_allowed_fallback(|| async { ApiError::method_not_allowed() })
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
