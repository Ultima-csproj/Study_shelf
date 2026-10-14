use super::references::{DisciplineDto, TeacherDto};
use serde::Serialize;
use study_core::catalog::{EducationalPlacement, MaterialSummary};
use study_core::material::{ContributorRole, RevisionCredit};
use study_core::timestamp::{TimestampError, format_utc};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialSummaryDto {
    id: i64,
    published_revision_id: i64,
    title: String,
    description: String,
    #[serde(rename = "type")]
    material_type: &'static str,
    tags: Vec<String>,
    credits: Vec<CreditDto>,
    placements: Vec<PlacementDto>,
    source_year: Option<i32>,
    published_at: String,
    first_published_at: String,
    preview: Option<String>,
    duration_seconds: Option<u32>,
    file_size_bytes: Option<i64>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditDto {
    contributor_id: i64,
    display_name: String,
    role: &'static str,
}
impl From<RevisionCredit> for CreditDto {
    fn from(value: RevisionCredit) -> Self {
        let role = match value.role {
            ContributorRole::Author => "author",
            ContributorRole::CoAuthor => "co_author",
            ContributorRole::Editor => "editor",
            ContributorRole::Translator => "translator",
            ContributorRole::Lecturer => "lecturer",
            ContributorRole::Reviewer => "reviewer",
        };
        Self {
            contributor_id: value.contributor_id.0,
            display_name: value.display_name,
            role,
        }
    }
}
#[derive(Serialize)]
pub struct PlacementDto {
    id: i64,
    discipline: DisciplineDto,
    responsible: Option<TeacherDto>,
    course: Option<u8>,
}
impl From<EducationalPlacement> for PlacementDto {
    fn from(value: EducationalPlacement) -> Self {
        Self {
            id: value.id.0,
            discipline: value.discipline.into(),
            responsible: value.responsible.map(Into::into),
            course: value.course,
        }
    }
}
impl TryFrom<MaterialSummary> for MaterialSummaryDto {
    type Error = TimestampError;
    fn try_from(value: MaterialSummary) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.0,
            published_revision_id: value.published_revision_id.0,
            title: value.title,
            description: value.description,
            material_type: value.material_type.as_db_value(),
            tags: value.tags,
            credits: value.credits.into_iter().map(Into::into).collect(),
            placements: value.placements.into_iter().map(Into::into).collect(),
            source_year: value.metadata.source_year,
            published_at: format_utc(value.published_at)?,
            first_published_at: format_utc(value.first_published_at)?,
            preview: value.metadata.preview_url,
            duration_seconds: value.metadata.duration_seconds,
            file_size_bytes: value.metadata.file_size_bytes,
        })
    }
}
#[derive(Serialize)]
pub struct MaterialDetailDto {
    #[serde(flatten)]
    summary: MaterialSummaryDto,
}
impl TryFrom<MaterialSummary> for MaterialDetailDto {
    type Error = TimestampError;
    fn try_from(value: MaterialSummary) -> Result<Self, Self::Error> {
        Ok(Self {
            summary: value.try_into()?,
        })
    }
}
