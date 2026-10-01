mod dto;
mod request;
mod response;

use axum::Json;
use axum::extract::{Path, RawQuery, State, rejection::PathRejection};
use study_core::ids::MaterialId;

use crate::api::error::ApiError;
use crate::api::state::AppState;

use self::dto::material::CatalogMaterialDto;
use self::request::parse_catalog_query;
use self::response::CatalogResponse;

pub async fn list(
    State(state): State<AppState>,
    RawQuery(raw_query): RawQuery,
) -> Result<Json<CatalogResponse>, ApiError> {
    let query = parse_catalog_query(raw_query.as_deref())?;

    let view = state.catalog.list(query).await.map_err(ApiError::from)?;

    Ok(Json(view.into()))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    path: Result<Path<String>, PathRejection>,
) -> Result<Json<CatalogMaterialDto>, ApiError> {
    let Path(id) = path.map_err(|_| ApiError::bad_request("Некорректный ID в пути запроса"))?;

    let id: i64 = id
        .parse()
        .map_err(|_| ApiError::bad_request("ID материала должен быть целым числом"))?;

    if id <= 0 {
        return Err(ApiError::bad_request(
            "ID материала должен быть положительным",
        ));
    }

    let item = state
        .catalog
        .get_by_id(MaterialId(id))
        .await
        .map_err(ApiError::from)?
        .ok_or_else(|| ApiError::not_found("Материал не найден"))?;

    Ok(Json(item.into()))
}
