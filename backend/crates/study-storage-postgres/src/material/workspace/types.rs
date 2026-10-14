use serde::Serialize;
use sqlx::FromRow;
use study_core::ids::InstitutionId;
use study_core::material::{MaterialLifecycle, PublicationState};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MaterialHeader {
    pub id: i64,
    pub title: String,
    pub lifecycle: String,
    pub publication: String,
    pub visibility: String,
    pub version: i64,
    pub owner_user_id: Option<i64>,
    pub owner_institution_id: Option<i64>,
    pub owner_name: String,
    pub published_revision_id: Option<i64>,
    pub working_revision_id: Option<i64>,
    pub working_revision_state: Option<String>,
    pub material_type: Option<String>,
    pub source_year: Option<i32>,
    pub description: Option<String>,
    pub updated_at: Option<String>,
    pub published_revision_number: Option<i64>,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RevisionHeader {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub state: String,
    pub version: i64,
    pub base_revision_id: Option<i64>,
    pub created_at: String,
    pub published_at: Option<String>,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionRow {
    pub id: i64,
    pub institution_id: Option<i64>,
    pub institution_name: Option<String>,
    pub reason: String,
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionTarget {
    pub material: MaterialHeader,
    pub global: bool,
    pub institutions: Vec<i64>,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlacementRow {
    pub id: i64,
    pub discipline_id: i64,
    pub discipline_name: String,
    pub institution_id: i64,
    pub institution_name: String,
    pub entry_count: i64,
    pub can_detach: bool,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlacementEntryRow {
    pub id: i64,
    pub section_id: i64,
    pub section_title: String,
    pub position: i32,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CollaboratorRow {
    pub user_id: i64,
    pub name: String,
    pub active: bool,
    pub permissions: Vec<String>,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MaterialEvent {
    pub id: i64,
    pub revision_id: Option<i64>,
    pub actor_name: String,
    pub action: String,
    pub reason: String,
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialCapabilities {
    pub manage: bool,
    pub edit: bool,
    pub submit: bool,
    pub collaborators: bool,
    pub restrict_global: bool,
    pub restrict_institutions: Vec<i64>,
}

#[derive(Serialize)]
pub struct Workspace {
    pub material: MaterialHeader,
    pub revisions: crate::administration::Page<RevisionHeader>,
    pub restrictions: Vec<RestrictionRow>,
    pub placements: Vec<PlacementRow>,
    pub capabilities: MaterialCapabilities,
}

pub struct RevisionView {
    pub material: MaterialHeader,
    pub stored: super::super::StoredRevision,
    pub outdated: bool,
    pub self_review: bool,
    pub can_moderate: bool,
    pub number: i64,
}

pub enum MaterialStateChange {
    Lifecycle(MaterialLifecycle),
    Publication(PublicationState),
    Private,
}

pub enum RevisionTransition {
    Discard,
    Submit,
    Withdraw,
    Approve,
    Reject,
}

#[derive(Clone, Copy)]
pub enum MaterialListKind {
    Author,
    Pending,
    Restricted,
}

#[derive(Clone, Copy, Default, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialListOrder {
    #[default]
    Newest,
    Oldest,
    Title,
}

impl MaterialListOrder {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Newest => "newest",
            Self::Oldest => "oldest",
            Self::Title => "title",
        }
    }
}

pub struct ContributorInput {
    pub id: i64,
    pub role: study_core::material::ContributorRole,
}

pub struct CollaboratorChange {
    pub user: study_core::ids::UserId,
    pub active: bool,
    pub permissions: Vec<study_core::material::MaterialPermission>,
}

pub struct ContentInput {
    pub title: String,
    pub description: String,
    pub material_type: study_core::catalog::MaterialType,
    pub metadata: study_core::material::RevisionMetadata,
    pub tags: Vec<String>,
    pub credits: Vec<ContributorInput>,
}

pub struct MaterialContext {
    pub restriction_id: Option<i64>,
    pub discipline_id: Option<i64>,
    pub collection_id: Option<i64>,
}

pub struct PublishedView {
    pub summary: study_core::catalog::MaterialSummary,
    pub lifecycle: String,
    pub publication: String,
    pub restrictions: Vec<RestrictionRow>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedHistory {
    pub material_id: i64,
    pub lifecycle: String,
    pub current_revision_id: Option<i64>,
    pub revisions: crate::administration::Page<RevisionHeader>,
}

pub struct NewMaterialInput {
    pub institution: Option<InstitutionId>,
    pub content: ContentInput,
}
