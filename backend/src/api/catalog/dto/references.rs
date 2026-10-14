use serde::Serialize;
use study_core::catalog::{DisciplineFacet, DisciplineReference, TeacherFacet, TeacherReference};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherDto {
    id: i64,
    name: String,
    short_name: String,
}
impl From<TeacherReference> for TeacherDto {
    fn from(value: TeacherReference) -> Self {
        Self {
            id: value.id.0,
            name: value.name,
            short_name: value.short_name,
        }
    }
}
#[derive(Serialize)]
pub struct DisciplineDto {
    id: i64,
    slug: String,
    name: String,
}
impl From<DisciplineReference> for DisciplineDto {
    fn from(value: DisciplineReference) -> Self {
        Self {
            id: value.id.0,
            slug: value.slug,
            name: value.name,
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
        Self {
            teacher: value.teacher.into(),
            material_count: value.material_count,
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
        Self {
            discipline: value.discipline.into(),
            material_count: value.material_count,
        }
    }
}
