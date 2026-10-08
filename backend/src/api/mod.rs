mod account;
mod administration;
mod catalog;
pub mod error;
mod learning;
mod material_workspace;
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
        .route(
            "/api/me/author-materials",
            get(material_workspace::author_list).post(material_workspace::create),
        )
        .route(
            "/api/admin/materials/pending",
            get(material_workspace::moderation_list),
        )
        .route(
            "/api/admin/materials/restricted",
            get(material_workspace::restricted_list),
        )
        .route(
            "/api/admin/materials/{id}/restriction-target",
            get(material_workspace::restriction_target),
        )
        .route(
            "/api/me/materials/{id}/workspace",
            get(material_workspace::workspace),
        )
        .route(
            "/api/me/materials/{id}/drafts",
            post(material_workspace::fork),
        )
        .route(
            "/api/me/materials/{id}/revisions/{revision}",
            get(material_workspace::revision).put(material_workspace::save),
        )
        .route(
            "/api/me/materials/{id}/revisions/{revision}/transition",
            post(material_workspace::transition),
        )
        .route(
            "/api/me/materials/{id}/state",
            post(material_workspace::material_state),
        )
        .route(
            "/api/me/materials/{id}/restrictions",
            post(material_workspace::restrict).get(material_workspace::restriction_metadata),
        )
        .route(
            "/api/me/materials/{id}/restrictions/{restriction}/revoke",
            post(material_workspace::unrestrict),
        )
        .route(
            "/api/me/materials/{id}/collaborators",
            get(material_workspace::collaborators),
        )
        .route(
            "/api/me/materials/{id}/collaborators/{user}",
            put(material_workspace::set_collaborator),
        )
        .route(
            "/api/me/materials/{id}/events",
            get(material_workspace::events),
        )
        .route(
            "/api/me/materials/{id}/placements/{placement}/detach",
            post(material_workspace::detach_placement),
        )
        .route(
            "/api/me/materials/{id}/placements/{placement}/entries",
            get(material_workspace::placement_entries),
        )
        .route(
            "/api/me/materials/{id}/entries/{entry}/detach",
            post(material_workspace::detach_entry),
        )
        .route(
            "/api/materials/{id}/history",
            get(material_workspace::history_public),
        )
        .route(
            "/api/me/materials/{id}/history",
            get(material_workspace::history_authenticated),
        )
        .route(
            "/api/materials/{id}/history/{revision}",
            get(material_workspace::history_revision_public),
        )
        .route(
            "/api/me/materials/{id}/history/{revision}",
            get(material_workspace::history_revision_authenticated),
        )
        .route("/api/admin/access", get(administration::access))
        .route("/api/admin/overview", get(administration::overview))
        .route("/api/admin/users", get(administration::users))
        .route("/api/admin/users/{id}", get(administration::user))
        .route(
            "/api/admin/users/{id}/state",
            post(administration::user_state),
        )
        .route(
            "/api/admin/users/{id}/sessions/revoke",
            post(administration::revoke_sessions),
        )
        .route(
            "/api/admin/users/{id}/permissions",
            put(administration::platform_permissions),
        )
        .route("/api/admin/institutions", get(administration::institutions))
        .route(
            "/api/admin/institutions/{id}",
            get(administration::institution).patch(administration::rename),
        )
        .route(
            "/api/admin/institutions/{id}/members",
            get(administration::members),
        )
        .route(
            "/api/admin/institutions/{institution}/members/{user}",
            put(administration::membership),
        )
        .route(
            "/api/admin/institutions/{institution}/requests/{user}/decision",
            post(administration::decide_membership),
        )
        .route(
            "/api/admin/requests/contributors",
            get(administration::contributor_requests),
        )
        .route(
            "/api/admin/requests/contributors/{contributor}/{user}/decision",
            post(administration::decide_contributor),
        )
        .route(
            "/api/admin/requests/institutions",
            get(administration::institution_requests),
        )
        .route(
            "/api/admin/requests/institutions/{id}/decision",
            post(administration::decide_institution),
        )
        .route("/api/admin/audit", get(administration::audit))
        .route(
            "/api/me/institution-requests",
            get(administration::own_institution_requests).post(administration::request_institution),
        )
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
        .route(
            "/api/me/materials/{id}",
            get(material_workspace::published_authenticated),
        )
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
        .route(
            "/api/materials/{id}",
            get(material_workspace::published_public),
        )
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
