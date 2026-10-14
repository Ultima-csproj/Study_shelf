use crate::api::{
    catalog::dto::material::{CreditDto, MaterialDetailDto},
    error::ApiError,
};
use serde::{Deserialize, Serialize};
use study_core::material::{ContributorRole, RevisionMetadata};
use study_storage_postgres::material::{
    ContentInput, ContributorInput, PublishedView, RevisionView,
};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditRole {
    Author,
    CoAuthor,
    Editor,
    Translator,
    Lecturer,
    Reviewer,
}
impl From<CreditRole> for ContributorRole {
    fn from(value: CreditRole) -> Self {
        match value {
            CreditRole::Author => Self::Author,
            CreditRole::CoAuthor => Self::CoAuthor,
            CreditRole::Editor => Self::Editor,
            CreditRole::Translator => Self::Translator,
            CreditRole::Lecturer => Self::Lecturer,
            CreditRole::Reviewer => Self::Reviewer,
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreditInput {
    pub contributor_id: i64,
    pub role: CreditRole,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContentBody {
    pub title: String,
    pub description: String,
    pub material_type: String,
    pub source_year: Option<i32>,
    pub preview_url: Option<String>,
    pub duration_seconds: Option<u32>,
    pub file_size_bytes: Option<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub credits: Vec<CreditInput>,
}
impl ContentBody {
    pub fn input(self) -> Result<ContentInput, ApiError> {
        let kind = study_core::catalog::MaterialType::from_db_value(&self.material_type)
            .ok_or_else(|| ApiError::bad_request("Неизвестный тип материала"))?;
        Ok(ContentInput {
            title: self.title,
            description: self.description,
            material_type: kind,
            metadata: RevisionMetadata {
                source_year: self.source_year,
                preview_url: self.preview_url,
                duration_seconds: self.duration_seconds,
                file_size_bytes: self.file_size_bytes,
            },
            tags: self.tags,
            credits: self
                .credits
                .into_iter()
                .map(|credit| ContributorInput {
                    id: credit.contributor_id,
                    role: credit.role.into(),
                })
                .collect(),
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionDto {
    id: i64,
    number: i64,
    material_id: i64,
    version: i64,
    state: &'static str,
    base_revision_id: Option<i64>,
    title: String,
    description: String,
    material_type: &'static str,
    source_year: Option<i32>,
    preview_url: Option<String>,
    duration_seconds: Option<u32>,
    file_size_bytes: Option<i64>,
    tags: Vec<String>,
    credits: Vec<CreditDto>,
    created_at: String,
    updated_at: String,
    submitted_at: Option<String>,
    published_at: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionDto {
    reviewer_id: i64,
    decision: &'static str,
    reason: Option<String>,
    reviewed_at: String,
    was_self_review: bool,
}
impl DecisionDto {
    pub fn new(action: study_core::material::ModerationAction) -> Result<Self, ApiError> {
        Ok(Self {
            reviewer_id: action.reviewer_id().0,
            decision: match action.decision() {
                study_core::material::ModerationDecision::Approved => "approved",
                study_core::material::ModerationDecision::Rejected => "rejected",
            },
            reason: action.reason().map(str::to_owned),
            reviewed_at: study_core::timestamp::format_utc(action.reviewed_at())
                .map_err(ApiError::internal)?,
            was_self_review: action.was_self_review(),
        })
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionResponse {
    material: study_storage_postgres::material::MaterialHeader,
    revision: RevisionDto,
    outdated: bool,
    self_review: bool,
    can_moderate: bool,
    decision: Option<DecisionDto>,
}
impl RevisionResponse {
    pub fn new(value: RevisionView, decision: Option<DecisionDto>) -> Result<Self, ApiError> {
        let revision = value.stored.revision;
        let content = revision.content();
        let utc = |at| study_core::timestamp::format_utc(at).map_err(ApiError::internal);
        let dto = RevisionDto {
            id: revision.id().0,
            number: value.number,
            material_id: revision.material_id().0,
            version: value.stored.edit_version,
            state: match revision.state() {
                study_core::material::RevisionState::Draft => "draft",
                study_core::material::RevisionState::PendingModeration => "pending_moderation",
                study_core::material::RevisionState::Published => "published",
                study_core::material::RevisionState::Rejected => "rejected",
                study_core::material::RevisionState::Discarded => "discarded",
            },
            base_revision_id: revision.base_revision_id().map(|id| id.0),
            title: content.title.clone(),
            description: content.description.clone(),
            material_type: content.material_type.as_db_value(),
            source_year: content.metadata.source_year,
            preview_url: content.metadata.preview_url.clone(),
            duration_seconds: content.metadata.duration_seconds,
            file_size_bytes: content.metadata.file_size_bytes,
            tags: content.tags.clone(),
            credits: content.credits.iter().cloned().map(Into::into).collect(),
            created_at: utc(revision.created_at())?,
            updated_at: utc(revision.updated_at())?,
            submitted_at: revision.submitted_at().map(utc).transpose()?,
            published_at: revision.published_at().map(utc).transpose()?,
        };
        Ok(Self {
            material: value.material,
            revision: dto,
            outdated: value.outdated,
            self_review: value.self_review,
            can_moderate: value.can_moderate,
            decision,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedResponse {
    #[serde(flatten)]
    detail: MaterialDetailDto,
    lifecycle: String,
    publication: String,
    restrictions: Vec<study_storage_postgres::material::RestrictionRow>,
}
impl PublishedResponse {
    pub fn new(value: PublishedView) -> Result<Self, ApiError> {
        Ok(Self {
            detail: value.summary.try_into().map_err(ApiError::internal)?,
            lifecycle: value.lifecycle,
            publication: value.publication,
            restrictions: value.restrictions,
        })
    }
}
