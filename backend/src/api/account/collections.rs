use super::{
    AccountJson,
    library::{PageDto, Pagination},
};
use crate::api::catalog::dto::material::MaterialSummaryDto;
use crate::{
    api::{error::ApiError, state::AppState},
    auth::{extractor::Authenticated, session},
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use study_storage_postgres::account::collections::{
    CollectionEdit, CollectionInfo, OwnedCollection,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InfoBody {
    title: String,
    #[serde(default)]
    short_description: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    purpose: String,
    #[serde(default)]
    target_audience: String,
}
impl From<InfoBody> for CollectionInfo {
    fn from(v: InfoBody) -> Self {
        Self {
            title: v.title,
            short_description: v.short_description,
            description: v.description,
            purpose: v.purpose,
            target_audience: v.target_audience,
        }
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedDto {
    id: i64,
    title: String,
    short_description: String,
    description: String,
    purpose: String,
    target_audience: String,
    visibility: String,
    edit_version: i64,
}
impl From<OwnedCollection> for OwnedDto {
    fn from(v: OwnedCollection) -> Self {
        Self {
            id: v.id,
            title: v.title,
            short_description: v.short_description,
            description: v.description,
            purpose: v.purpose,
            target_audience: v.target_audience,
            visibility: v.visibility,
            edit_version: v.edit_version,
        }
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionDto {
    id: i64,
    title: String,
    description: String,
    position: i32,
    entry_count: i64,
}
#[derive(Serialize)]
pub struct DetailDto {
    collection: OwnedDto,
    sections: Vec<SectionDto>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDto {
    id: i64,
    material_id: i64,
    position: i32,
    material: Option<MaterialSummaryDto>,
}
pub async fn list(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<Pagination>,
) -> Result<Json<PageDto<OwnedDto>>, ApiError> {
    let result = state
        .auth
        .repository
        .owned_collections(&actor.digest, p.page, p.page_size)
        .await?;
    Ok(Json(PageDto::new(
        result.items.into_iter().map(Into::into).collect(),
        result.total,
        &p,
    )))
}
pub async fn create(
    State(state): State<AppState>,
    actor: Authenticated,
    AccountJson(body): AccountJson<InfoBody>,
) -> Result<Json<DetailDto>, ApiError> {
    let id = state
        .auth
        .repository
        .create_collection(
            &actor.digest,
            &body.into(),
            &format!("personal-{}", session::secret()?),
        )
        .await?;
    detail(State(state), actor, Path(id)).await
}
pub async fn detail(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<DetailDto>, ApiError> {
    let result = state
        .auth
        .repository
        .owned_detail(&actor.digest, id)
        .await?;
    Ok(Json(DetailDto {
        collection: result.collection.into(),
        sections: result
            .sections
            .into_iter()
            .map(|s| SectionDto {
                id: s.id,
                title: s.title,
                description: s.description,
                position: s.position,
                entry_count: s.entry_count,
            })
            .collect(),
    }))
}
pub async fn entries(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((id, section)): Path<(i64, i64)>,
    Query(p): Query<Pagination>,
) -> Result<Json<PageDto<EntryDto>>, ApiError> {
    let result = state
        .auth
        .repository
        .owned_entries(&actor.digest, id, section, p.page, p.page_size)
        .await?;
    let mut materials = result
        .materials
        .into_iter()
        .map(|m| Ok((m.id.0, m.try_into().map_err(ApiError::internal)?)))
        .collect::<Result<std::collections::HashMap<_, MaterialSummaryDto>, ApiError>>()?;
    Ok(Json(PageDto::new(
        result
            .entries
            .into_iter()
            .map(|e| EntryDto {
                id: e.id,
                material_id: e.material_id,
                position: e.position,
                material: materials.remove(&e.material_id),
            })
            .collect(),
        result.total,
        &p,
    )))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EditBody {
    expected_version: i64,
    change: EditAction,
}
#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum EditAction {
    Info(InfoBody),
    AddSection {
        title: String,
        #[serde(default)]
        description: String,
    },
    UpdateSection {
        id: i64,
        title: String,
        #[serde(default)]
        description: String,
    },
    DeleteSection {
        id: i64,
    },
    AddEntry {
        section: i64,
        material: i64,
    },
    DeleteEntry {
        section: i64,
        id: i64,
    },
    ReorderSections {
        ids: Vec<i64>,
    },
    ReorderEntries {
        section: i64,
        ids: Vec<i64>,
    },
}
pub async fn edit(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
    AccountJson(body): AccountJson<EditBody>,
) -> Result<Json<DetailDto>, ApiError> {
    let action = match body.change {
        EditAction::Info(v) => CollectionEdit::Info(v.into()),
        EditAction::AddSection { title, description } => {
            CollectionEdit::AddSection { title, description }
        }
        EditAction::UpdateSection {
            id,
            title,
            description,
        } => CollectionEdit::UpdateSection {
            id,
            title,
            description,
        },
        EditAction::DeleteSection { id } => CollectionEdit::DeleteSection { id },
        EditAction::AddEntry { section, material } => {
            CollectionEdit::AddEntry { section, material }
        }
        EditAction::DeleteEntry { section, id } => CollectionEdit::DeleteEntry { section, id },
        EditAction::ReorderSections { ids } => CollectionEdit::ReorderSections { ids },
        EditAction::ReorderEntries { section, ids } => {
            CollectionEdit::ReorderEntries { section, ids }
        }
    };
    state
        .auth
        .repository
        .edit_collection(&actor.digest, id, body.expected_version, action)
        .await?;
    detail(State(state), actor, Path(id)).await
}
