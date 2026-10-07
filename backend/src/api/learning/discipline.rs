use super::{
    dto::{DetailDto, PageDto, SectionPageDto},
    request,
};
use crate::api::catalog::dto::references::TeacherDto;
use crate::api::{error::ApiError, state::AppState};
use axum::{
    Json,
    extract::{Path, RawQuery, State},
};
use serde::Serialize;
use study_core::discipline::DisciplineSummary;
use study_core::ids::{DisciplineId, SectionId};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DisciplineDto {
    id: i64,
    slug: String,
    name: String,
    description: String,
    summary: String,
    institution: InstitutionDto,
    material_count: u64,
    step_count: u64,
    courses: Vec<u8>,
    responsible: Vec<TeacherDto>,
}
#[derive(Serialize)]
struct InstitutionDto {
    id: i64,
    name: String,
}
impl From<DisciplineSummary> for DisciplineDto {
    fn from(value: DisciplineSummary) -> Self {
        Self {
            id: value.id.0,
            slug: value.slug,
            name: value.name,
            description: value.description,
            summary: value.summary,
            institution: InstitutionDto {
                id: value.institution.id.0,
                name: value.institution.name,
            },
            material_count: value.material_count,
            step_count: value.step_count,
            courses: value.courses,
            responsible: value.responsible.into_iter().map(Into::into).collect(),
        }
    }
}
pub async fn list(
    State(state): State<AppState>,
    RawQuery(raw): RawQuery,
) -> Result<Json<PageDto<DisciplineDto>>, ApiError> {
    let query = request::parse(raw.as_deref(), false)?;
    let page = state.disciplines.list(&query).await?;
    Ok(Json(
        PageDto::map(page, |item| Ok(item.into())).map_err(ApiError::internal)?,
    ))
}
pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DetailDto<DisciplineDto>>, ApiError> {
    let item = state
        .disciplines
        .get_by_id(DisciplineId(request::id(&id)?))
        .await?
        .ok_or_else(|| ApiError::not_found("Дисциплина не найдена"))?;
    Ok(Json(
        DetailDto::new(item.summary.into(), item.sections, item.materials)
            .map_err(ApiError::internal)?,
    ))
}
pub async fn items(
    State(state): State<AppState>,
    Path((id, section)): Path<(String, String)>,
    RawQuery(raw): RawQuery,
) -> Result<Json<SectionPageDto>, ApiError> {
    let pagination = request::pagination(raw.as_deref())?;
    let item = state
        .disciplines
        .section_items(
            DisciplineId(request::id(&id)?),
            SectionId(request::id(&section)?),
            pagination,
        )
        .await?
        .ok_or_else(|| ApiError::not_found("Раздел не найден"))?;
    Ok(Json(item.try_into().map_err(ApiError::internal)?))
}
