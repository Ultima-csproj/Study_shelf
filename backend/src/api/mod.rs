mod account;
mod catalog;
pub mod error;
mod learning;
pub mod state;

use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, post, put},
};
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
        .route("/api/auth/csrf", get(account::auth::csrf))
        .route("/api/auth/register", post(account::auth::register))
        .route("/api/auth/login", post(account::auth::login))
        .route("/api/auth/logout", post(account::auth::logout))
        .route(
            "/api/auth/logout-others",
            post(account::auth::logout_others),
        )
        .route("/api/viewer", get(account::auth::viewer))
        .route(
            "/api/me/library/contributions",
            get(account::library::contributions),
        )
        .route("/api/institutions", get(account::profile::institutions))
        .route(
            "/api/me/institutions/{institution}/membership-requests/{user}/decision",
            post(account::identity::membership),
        )
        .route(
            "/api/me/contributor-requests/{contributor}/users/{user}/decision",
            post(account::identity::contributor),
        )
        .route(
            "/api/me/profile",
            get(account::profile::get).patch(account::profile::update),
        )
        .route("/api/me/password", post(account::profile::password))
        .route("/api/me/identity-requests", get(account::profile::requests))
        .route(
            "/api/me/membership-requests/{id}",
            put(account::profile::membership),
        )
        .route(
            "/api/me/contributor-requests/{id}",
            put(account::profile::contributor),
        )
        .route("/api/me/materials/{id}", get(account::profile::material))
        .route(
            "/api/me/library/materials",
            get(account::library::materials),
        )
        .route(
            "/api/me/library/collections",
            get(account::library::collections),
        )
        .route("/api/me/library/flags", post(account::library::flags))
        .route(
            "/api/me/library/materials/{id}",
            put(account::library::save_material).delete(account::library::unsave_material),
        )
        .route(
            "/api/me/library/collections/{id}",
            put(account::library::save_collection).delete(account::library::unsave_collection),
        )
        .route(
            "/api/me/collections",
            get(account::collections::list).post(account::collections::create),
        )
        .route(
            "/api/me/collections/{id}",
            get(account::collections::detail).patch(account::collections::edit),
        )
        .route(
            "/api/me/collections/{id}/sections/{section}/items",
            get(account::collections::entries),
        )
        .route("/api/catalog", get(catalog::list))
        .route("/api/materials/{id}", get(catalog::get_by_id))
        .route("/api/disciplines", get(learning::discipline::list))
        .route("/api/disciplines/{id}", get(learning::discipline::detail))
        .route(
            "/api/disciplines/{id}/sections/{section}/items",
            get(learning::discipline::items),
        )
        .route("/api/collections", get(learning::collection::list))
        .route("/api/collections/{id}", get(learning::collection::detail))
        .route(
            "/api/collections/{id}/sections/{section}/items",
            get(learning::collection::items),
        )
        .fallback(|| async { ApiError::not_found("Маршрут не найден") })
        .method_not_allowed_fallback(|| async { ApiError::method_not_allowed() })
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(64 * 1024))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::session::guard,
        ))
        .with_state(state)
}
