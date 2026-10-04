use super::mapping;
use sqlx::FromRow;
use study_core::error::{RepositoryError, RepositoryResult};
use study_core::ids::{InstitutionId, MaterialId, MaterialRevisionId, UserId};
use study_core::material::{
    Material, MaterialOwner, MaterialRevision, MaterialSnapshot, RevisionContent, RevisionMetadata,
    RevisionSnapshot,
};
use time::OffsetDateTime;

#[derive(FromRow)]
pub(super) struct MaterialRow {
    pub id: i64,
    pub owner_user_id: Option<i64>,
    pub owner_institution_id: Option<i64>,
    pub created_by_user_id: i64,
    pub created_at: OffsetDateTime,
    pub visibility: String,
    pub published_revision_id: Option<i64>,
    pub published_at: Option<OffsetDateTime>,
    pub published_state: Option<String>,
}
impl MaterialRow {
    pub fn into_domain(self) -> RepositoryResult<Material> {
        let owner = match (self.owner_user_id, self.owner_institution_id) {
            (Some(id), None) => MaterialOwner::User(UserId(id)),
            (None, Some(id)) => MaterialOwner::Institution(InstitutionId(id)),
            _ => return Err(crate::error::invalid_relation("material owner")),
        };
        let published_revision = match (
            self.published_revision_id,
            self.published_at,
            self.published_state.as_deref(),
        ) {
            (Some(id), Some(at), Some("published")) => {
                Some((MaterialRevisionId(id), mapping::stored_time(at)?))
            }
            (None, None, None) => None,
            _ => return Err(crate::error::invalid_relation("published revision")),
        };
        Material::try_restore(MaterialSnapshot {
            id: MaterialId(self.id),
            owner,
            created_by_user_id: UserId(self.created_by_user_id),
            created_at: mapping::stored_time(self.created_at)?,
            visibility: mapping::visibility(&self.visibility)?,
            published_revision,
        })
        .map_err(|error| {
            RepositoryError::invalid_data_with_source("invalid material snapshot", error)
        })
    }
}

#[derive(FromRow)]
pub(super) struct RevisionRow {
    pub id: i64,
    pub material_id: i64,
    pub base_revision_id: Option<i64>,
    pub created_by_user_id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub state: String,
    pub edit_version: i64,
    pub title: String,
    pub description: String,
    pub material_type: String,
    pub source_year: Option<i32>,
    pub preview_url: Option<String>,
    pub duration_seconds: Option<i64>,
    pub file_size_bytes: Option<i64>,
    pub submitted_by_user_id: Option<i64>,
    pub submitted_at: Option<OffsetDateTime>,
    pub published_at: Option<OffsetDateTime>,
}
impl RevisionRow {
    pub fn into_domain(
        self,
        tags: Vec<String>,
        credits: Vec<study_core::material::RevisionCredit>,
        editors: Vec<UserId>,
    ) -> RepositoryResult<MaterialRevision> {
        let duration_seconds = self
            .duration_seconds
            .map(u32::try_from)
            .transpose()
            .map_err(|error| {
                RepositoryError::invalid_data_with_source("invalid duration", error)
            })?;
        MaterialRevision::try_restore(RevisionSnapshot {
            id: MaterialRevisionId(self.id),
            material_id: MaterialId(self.material_id),
            base_revision_id: self.base_revision_id.map(MaterialRevisionId),
            created_by_user_id: UserId(self.created_by_user_id),
            created_at: mapping::stored_time(self.created_at)?,
            updated_at: mapping::stored_time(self.updated_at)?,
            state: mapping::state(&self.state)?,
            content: RevisionContent {
                title: self.title,
                description: self.description,
                material_type: mapping::type_from_db(&self.material_type)?,
                metadata: RevisionMetadata {
                    source_year: self.source_year,
                    preview_url: self.preview_url,
                    duration_seconds,
                    file_size_bytes: self.file_size_bytes,
                },
                tags,
                credits,
            },
            changed_by: editors,
            submitted_by_user_id: self.submitted_by_user_id.map(UserId),
            submitted_at: self.submitted_at.map(mapping::stored_time).transpose()?,
            published_at: self.published_at.map(mapping::stored_time).transpose()?,
        })
        .map_err(|error| {
            RepositoryError::invalid_data_with_source("invalid revision snapshot", error)
        })
    }
}
