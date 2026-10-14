pub(super) mod dto;
mod request;
mod response;

use axum::Json;
use axum::extract::{RawQuery, State};

use crate::api::error::ApiError;
use crate::api::state::AppState;

use self::request::parse_catalog_query;
use self::response::CatalogResponse;

pub async fn list(
    State(state): State<AppState>,
    RawQuery(raw_query): RawQuery,
) -> Result<Json<CatalogResponse>, ApiError> {
    let query = parse_catalog_query(raw_query.as_deref())?;

    let view = state.catalog.list(query).await.map_err(ApiError::from)?;

    Ok(Json(view.try_into().map_err(ApiError::internal)?))
}
