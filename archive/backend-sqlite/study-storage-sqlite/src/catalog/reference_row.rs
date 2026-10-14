use sqlx::FromRow;

use study_core::catalog::{DisciplineReference, TeacherReference};

use study_core::ids::{DisciplineId, TeacherId};

#[derive(Debug, FromRow)]
pub(super) struct TeacherRow {
    id: i64,
    name: String,
    short_name: String,
}

impl From<TeacherRow> for TeacherReference {
    fn from(row: TeacherRow) -> Self {
        TeacherReference {
            id: TeacherId(row.id),
            name: row.name,
            short_name: row.short_name,
        }
    }
}

#[derive(Debug, FromRow)]
pub(super) struct DisciplineRow {
    id: i64,
    slug: String,
    name: String,
    description: String,
    summary: Option<String>,
    course: i64,
    semester: i64,
    teacher_id: i64,
}

impl From<DisciplineRow> for DisciplineReference {
    fn from(row: DisciplineRow) -> Self {
        Self {
            id: DisciplineId(row.id),
            slug: row.slug,
            name: row.name,
            description: row.description,
            summary: row.summary,
            course: row.course,
            semester: row.semester,
            teacher_id: TeacherId(row.teacher_id),
        }
    }
}
