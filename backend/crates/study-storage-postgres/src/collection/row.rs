use sqlx::FromRow;
use study_core::catalog::DisciplineReference;
use study_core::collection::{CollectionDifficulty, CollectionSummary};
use study_core::error::RepositoryResult;
use study_core::ids::{CollectionId, DisciplineId};
use time::OffsetDateTime;

#[derive(FromRow)]
pub(super) struct CollectionRow {
    id: i64,
    slug: String,
    title: String,
    short_description: String,
    description: String,
    purpose: String,
    target_audience: String,
    discipline_id: Option<i64>,
    discipline_slug: Option<String>,
    discipline_name: Option<String>,
    course: Option<i16>,
    difficulty: Option<String>,
    estimated_minutes: Option<i32>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    step_count: i64,
    unique_material_count: i64,
}
impl CollectionRow {
    pub fn into_summary(self) -> RepositoryResult<CollectionSummary> {
        if self.id <= 0 || self.discipline_id.is_some_and(|id| id <= 0) {
            return Err(crate::error::invalid_relation("collection identity"));
        }
        let discipline = match (
            self.discipline_id,
            self.discipline_slug,
            self.discipline_name,
        ) {
            (Some(id), Some(slug), Some(name)) => Some(DisciplineReference {
                id: DisciplineId(id),
                slug,
                name,
            }),
            (None, None, None) => None,
            _ => return Err(crate::error::invalid_relation("collection discipline")),
        };
        let difficulty = match self.difficulty.as_deref() {
            Some("beginner") => Some(CollectionDifficulty::Beginner),
            Some("intermediate") => Some(CollectionDifficulty::Intermediate),
            Some("advanced") => Some(CollectionDifficulty::Advanced),
            None => None,
            Some(value) => return Err(crate::error::unknown("collection difficulty", value)),
        };
        let estimated_minutes = self
            .estimated_minutes
            .map(u32::try_from)
            .transpose()
            .map_err(|error| {
                study_core::error::RepositoryError::invalid_data_with_source(
                    "estimated minutes",
                    error,
                )
            })?;
        let created_at = crate::material::mapping::stored_time(self.created_at)?;
        let updated_at = crate::material::mapping::stored_time(self.updated_at)?;
        if updated_at < created_at {
            return Err(crate::error::invalid_relation("collection timestamps"));
        }
        Ok(CollectionSummary {
            id: CollectionId(self.id),
            slug: self.slug,
            title: self.title,
            short_description: self.short_description,
            description: self.description,
            purpose: self.purpose,
            target_audience: self.target_audience,
            discipline,
            course: self.course.map(crate::learning::course).transpose()?,
            difficulty,
            estimated_minutes,
            created_at,
            updated_at,
            contributors: Vec::new(),
            step_count: crate::learning::count(self.step_count)?,
            unique_material_count: crate::learning::count(self.unique_material_count)?,
        })
    }
}
