use super::{
    dto::{DetailDto, PageDto, SectionPageDto},
    request,
};
use crate::api::catalog::dto::references::DisciplineDto;
use crate::api::{error::ApiError, state::AppState};
use axum::{
    Json,
    extract::{Path, RawQuery, State},
};
use serde::Serialize;
use study_core::collection::{
    CollectionDifficulty, CollectionParticipant, CollectionRole, CollectionSummary,
};
use study_core::ids::{CollectionId, SectionId};
use study_core::timestamp::{TimestampError, format_utc};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CollectionDto {
    id: i64,
    slug: String,
    title: String,
    short_description: String,
    description: String,
    purpose: String,
    target_audience: String,
    discipline: Option<DisciplineDto>,
    course: Option<u8>,
    difficulty: Option<&'static str>,
    estimated_minutes: Option<u32>,
    created_at: String,
    updated_at: String,
    contributors: Vec<ParticipantDto>,
    step_count: u64,
    unique_material_count: u64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ParticipantDto {
    contributor_id: i64,
    display_name: String,
    role: &'static str,
    position: u64,
}
impl From<CollectionParticipant> for ParticipantDto {
    fn from(value: CollectionParticipant) -> Self {
        let role = match value.role {
            CollectionRole::Author => "author",
            CollectionRole::Curator => "curator",
            CollectionRole::Contributor => "contributor",
        };
        Self {
            contributor_id: value.contributor_id.0,
            display_name: value.display_name,
            role,
            position: value.position,
        }
    }
}
impl TryFrom<CollectionSummary> for CollectionDto {
    type Error = TimestampError;
    fn try_from(value: CollectionSummary) -> Result<Self, Self::Error> {
        let difficulty = value.difficulty.map(|value| match value {
            CollectionDifficulty::Beginner => "beginner",
            CollectionDifficulty::Intermediate => "intermediate",
            CollectionDifficulty::Advanced => "advanced",
        });
        Ok(Self {
            id: value.id.0,
            slug: value.slug,
            title: value.title,
            short_description: value.short_description,
            description: value.description,
            purpose: value.purpose,
            target_audience: value.target_audience,
            discipline: value.discipline.map(Into::into),
            course: value.course,
            difficulty,
            estimated_minutes: value.estimated_minutes,
            created_at: format_utc(value.created_at)?,
            updated_at: format_utc(value.updated_at)?,
            contributors: value.contributors.into_iter().map(Into::into).collect(),
            step_count: value.step_count,
            unique_material_count: value.unique_material_count,
        })
    }
}
#[derive(Serialize)]
pub(crate) struct CollectionResponse {
    #[serde(flatten)]
    page: PageDto<CollectionDto>,
    disciplines: Vec<DisciplineDto>,
    courses: Vec<u8>,
}
pub async fn list(
    State(state): State<AppState>,
    RawQuery(raw): RawQuery,
) -> Result<Json<CollectionResponse>, ApiError> {
    let query = request::parse(raw.as_deref(), true)?;
    let result = state.collections.list(&query).await?;
    Ok(Json(CollectionResponse {
        page: PageDto::map(result.page, TryInto::try_into).map_err(ApiError::internal)?,
        disciplines: result.disciplines.into_iter().map(Into::into).collect(),
        courses: result.courses,
    }))
}
pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DetailDto<CollectionDto>>, ApiError> {
    let item = state
        .collections
        .get_by_id(CollectionId(request::id(&id)?))
        .await?
        .ok_or_else(|| ApiError::not_found("Подборка не найдена"))?;
    let summary = item.summary.try_into().map_err(ApiError::internal)?;
    Ok(Json(
        DetailDto::new(summary, item.sections, item.materials).map_err(ApiError::internal)?,
    ))
}
pub async fn items(
    State(state): State<AppState>,
    Path((id, section)): Path<(String, String)>,
    RawQuery(raw): RawQuery,
) -> Result<Json<SectionPageDto>, ApiError> {
    let pagination = request::pagination(raw.as_deref())?;
    let item = state
        .collections
        .section_items(
            CollectionId(request::id(&id)?),
            SectionId(request::id(&section)?),
            pagination,
        )
        .await?
        .ok_or_else(|| ApiError::not_found("Раздел не найден"))?;
    Ok(Json(item.try_into().map_err(ApiError::internal)?))
}
