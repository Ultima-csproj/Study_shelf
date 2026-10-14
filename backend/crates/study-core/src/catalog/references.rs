use crate::ids::{DisciplineId, TeacherId};

/// Public identity of the responsible user in an educational placement.
#[derive(Debug, Clone)]
pub struct TeacherReference {
    pub id: TeacherId,
    pub name: String,
    pub short_name: String,
}

#[derive(Debug, Clone)]
pub struct DisciplineReference {
    pub id: DisciplineId,
    pub slug: String,
    pub name: String,
}
