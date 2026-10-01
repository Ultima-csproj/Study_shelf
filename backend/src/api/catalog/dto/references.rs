use serde::Serialize;
use study_storage_sqlite::catalog::{DisciplineFacet, TeacherFacet};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherDto {
    id: i64,
    name: String,
    short_name: String,
}

impl From<TeacherFacet> for TeacherDto {
    fn from(value: TeacherFacet) -> Self {
        Self {
            id: value.id,
            name: value.name,
            short_name: value.short_name,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisciplineDto {
    id: i64,
    slug: String,
    name: String,
    description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    course: i64,
    semester: i64,
    teacher_id: i64,
}

impl From<DisciplineFacet> for DisciplineDto {
    fn from(value: DisciplineFacet) -> Self {
        Self {
            id: value.id,
            slug: value.slug,
            name: value.name,
            description: value.description,
            summary: value.summary,
            course: value.course,
            semester: value.semester,
            teacher_id: value.teacher_id,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherFacetDto {
    #[serde(flatten)]
    teacher: TeacherDto,

    material_count: i64,
}

impl From<TeacherFacet> for TeacherFacetDto {
    fn from(value: TeacherFacet) -> Self {
        let material_count = value.material_count;

        Self {
            teacher: value.into(),
            material_count,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisciplineFacetDto {
    #[serde(flatten)]
    discipline: DisciplineDto,

    material_count: i64,
}

impl From<DisciplineFacet> for DisciplineFacetDto {
    fn from(value: DisciplineFacet) -> Self {
        let material_count = value.material_count;

        Self {
            discipline: value.into(),
            material_count,
        }
    }
}
