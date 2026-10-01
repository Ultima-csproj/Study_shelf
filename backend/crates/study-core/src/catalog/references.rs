use crate::ids::{DisciplineId, TeacherId};

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
    pub description: String,
    pub summary: Option<String>,
    pub course: i64,
    pub semester: i64,
    pub teacher_id: TeacherId,
}
