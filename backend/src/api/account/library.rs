use super::AccountJson;
use crate::api::catalog::dto::material::MaterialSummaryDto;
use crate::{
    api::{error::ApiError, state::AppState},
    auth::extractor::Authenticated,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use study_storage_postgres::account::library::SavedKind;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Pagination {
    #[serde(default = "first")]
    pub page: u32,
    #[serde(default = "size")]
    pub page_size: u32,
}
fn first() -> u32 {
    1
}
fn size() -> u32 {
    20
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_count: i64,
}
impl<T> PageDto<T> {
    pub fn new(items: Vec<T>, total: i64, p: &Pagination) -> Self {
        Self {
            items,
            total,
            page: p.page,
            page_count: (total + i64::from(p.page_size) - 1) / i64::from(p.page_size),
        }
    }
}
#[derive(Serialize)]
pub struct MaterialSlot {
    id: i64,
    material: Option<MaterialSummaryDto>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSlot {
    id: i64,
    title: Option<String>,
    short_description: Option<String>,
    owned: bool,
}
pub async fn materials(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<Pagination>,
) -> Result<Json<PageDto<MaterialSlot>>, ApiError> {
    let result = state
        .auth
        .repository
        .saved_materials(&actor.digest, p.page, p.page_size)
        .await?;
    let mut items = result
        .materials
        .into_iter()
        .map(|m| {
            Ok((
                m.id.0,
                MaterialSummaryDto::try_from(m).map_err(ApiError::internal)?,
            ))
        })
        .collect::<Result<std::collections::HashMap<_, _>, ApiError>>()?;
    let slots = result
        .ids
        .into_iter()
        .map(|id| MaterialSlot {
            id,
            material: items.remove(&id),
        })
        .collect();
    Ok(Json(PageDto::new(slots, result.total, &p)))
}
pub async fn collections(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<Pagination>,
) -> Result<Json<PageDto<CollectionSlot>>, ApiError> {
    let result = state
        .auth
        .repository
        .saved_collections(&actor.digest, p.page, p.page_size)
        .await?;
    Ok(Json(PageDto::new(
        result
            .items
            .into_iter()
            .map(|c| CollectionSlot {
                id: c.id,
                title: c.title,
                short_description: c.short_description,
                owned: c.owned,
            })
            .collect(),
        result.total,
        &p,
    )))
}
pub async fn contributions(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<Pagination>,
) -> Result<Json<PageDto<CollectionSlot>>, ApiError> {
    let result = state
        .auth
        .repository
        .contributed_collections(&actor.digest, p.page, p.page_size)
        .await?;
    Ok(Json(PageDto::new(
        result
            .items
            .into_iter()
            .map(|c| CollectionSlot {
                id: c.id,
                title: c.title,
                short_description: c.short_description,
                owned: c.owned,
            })
            .collect(),
        result.total,
        &p,
    )))
}
pub async fn save_material(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .auth
        .repository
        .save(&actor.digest, SavedKind::Material, id, true)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn unsave_material(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .auth
        .repository
        .save(&actor.digest, SavedKind::Material, id, false)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn save_collection(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .auth
        .repository
        .save(&actor.digest, SavedKind::Collection, id, true)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn unsave_collection(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .auth
        .repository
        .save(&actor.digest, SavedKind::Collection, id, false)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlagsBody {
    material_ids: Vec<i64>,
    collection_ids: Vec<i64>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagsDto {
    material_ids: Vec<i64>,
    collection_ids: Vec<i64>,
}
pub async fn flags(
    State(state): State<AppState>,
    actor: Authenticated,
    AccountJson(body): AccountJson<FlagsBody>,
) -> Result<Json<FlagsDto>, ApiError> {
    let (material_ids, collection_ids) = state
        .auth
        .repository
        .saved_flags(&actor.digest, &body.material_ids, &body.collection_ids)
        .await?;
    Ok(Json(FlagsDto {
        material_ids,
        collection_ids,
    }))
}
