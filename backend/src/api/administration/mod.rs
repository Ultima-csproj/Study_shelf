mod institutions;
mod requests;
mod users;
use crate::{
    api::{error::ApiError, state::AppState},
    auth::extractor::Authenticated,
};
use axum::{
    Json,
    extract::{Query, State},
};
pub use institutions::*;
pub use requests::*;
use serde::Deserialize;
use study_storage_postgres::administration::{AdminAccess, AuditRow, Page, PageRequest};
pub use users::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdminQuery {
    #[serde(default = "first")]
    page: u32,
    #[serde(default = "size")]
    page_size: u32,
    #[serde(default)]
    q: String,
    state: Option<String>,
}
fn first() -> u32 {
    1
}
fn size() -> u32 {
    20
}
impl From<AdminQuery> for PageRequest {
    fn from(p: AdminQuery) -> Self {
        Self {
            page: p.page,
            size: p.page_size,
            query: p.q,
            state: p.state,
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DecisionBody {
    pub expected_version: i64,
    pub accept: bool,
    pub reason: String,
}
pub async fn access(
    State(state): State<AppState>,
    actor: Authenticated,
) -> Result<Json<AdminAccess>, ApiError> {
    Ok(Json(state.administration.access(&actor.digest).await?))
}
pub async fn overview(
    State(state): State<AppState>,
    actor: Authenticated,
) -> Result<Json<study_storage_postgres::administration::Overview>, ApiError> {
    Ok(Json(state.administration.overview(&actor.digest).await?))
}
pub async fn audit(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<AdminQuery>,
) -> Result<Json<Page<AuditRow>>, ApiError> {
    Ok(Json(
        state.administration.audit(&actor.digest, &p.into()).await?,
    ))
}
