mod repository;
use crate::catalog::TeacherReference;
use crate::ids::DisciplineId;
use crate::learning::InstitutionReference;
pub use repository::DisciplineRepository;

#[derive(Debug)]
pub struct DisciplineSummary {
    pub id: DisciplineId,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub summary: String,
    pub institution: InstitutionReference,
    pub material_count: u64,
    pub step_count: u64,
    pub courses: Vec<u8>,
    pub responsible: Vec<TeacherReference>,
}
