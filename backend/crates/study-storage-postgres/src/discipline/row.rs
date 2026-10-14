use sqlx::FromRow;
use study_core::discipline::DisciplineSummary;
use study_core::error::RepositoryResult;
use study_core::ids::{DisciplineId, InstitutionId};
use study_core::learning::InstitutionReference;

#[derive(FromRow)]
pub(super) struct DisciplineRow {
    id: i64,
    slug: String,
    name: String,
    description: String,
    summary: String,
    institution_id: i64,
    institution_name: String,
    material_count: i64,
    step_count: i64,
    courses: Vec<i16>,
}
impl DisciplineRow {
    pub fn into_summary(self) -> RepositoryResult<DisciplineSummary> {
        if self.id <= 0 || self.institution_id <= 0 {
            return Err(crate::error::invalid_relation("discipline identity"));
        }
        Ok(DisciplineSummary {
            id: DisciplineId(self.id),
            slug: self.slug,
            name: self.name,
            description: self.description,
            summary: self.summary,
            institution: InstitutionReference {
                id: InstitutionId(self.institution_id),
                name: self.institution_name,
            },
            material_count: crate::learning::count(self.material_count)?,
            step_count: crate::learning::count(self.step_count)?,
            courses: self
                .courses
                .into_iter()
                .map(crate::learning::course)
                .collect::<Result<_, _>>()?,
            responsible: Vec::new(),
        })
    }
}
