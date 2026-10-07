use crate::material::mapping;
use sqlx::FromRow;
use study_core::catalog::MaterialSummary;
use study_core::error::{RepositoryError, RepositoryResult};
use study_core::ids::{MaterialId, MaterialRevisionId};
use study_core::material::RevisionMetadata;
use time::OffsetDateTime;

#[derive(FromRow)]
pub(crate) struct SummaryRow {
    id: i64,
    published_revision_id: i64,
    title: String,
    description: String,
    material_type: String,
    source_year: Option<i32>,
    preview_url: Option<String>,
    duration_seconds: Option<i64>,
    file_size_bytes: Option<i64>,
    first_published_at: OffsetDateTime,
    published_at: OffsetDateTime,
}
impl SummaryRow {
    pub fn into_summary(self) -> RepositoryResult<MaterialSummary> {
        if self.id <= 0 || self.published_revision_id <= 0 {
            return Err(crate::error::invalid_relation("public identifiers"));
        }
        let first = mapping::stored_time(self.first_published_at)?;
        let published = mapping::stored_time(self.published_at)?;
        if published < first {
            return Err(crate::error::invalid_relation("publication timestamps"));
        }
        Ok(MaterialSummary {
            id: MaterialId(self.id),
            published_revision_id: MaterialRevisionId(self.published_revision_id),
            title: self.title,
            description: self.description,
            material_type: mapping::type_from_db(&self.material_type)?,
            tags: Vec::new(),
            credits: Vec::new(),
            placements: Vec::new(),
            metadata: RevisionMetadata {
                source_year: self.source_year,
                preview_url: self.preview_url,
                duration_seconds: self
                    .duration_seconds
                    .map(u32::try_from)
                    .transpose()
                    .map_err(|error| {
                        RepositoryError::invalid_data_with_source("invalid duration", error)
                    })?,
                file_size_bytes: self.file_size_bytes,
            },
            first_published_at: first,
            published_at: published,
        })
    }
}
