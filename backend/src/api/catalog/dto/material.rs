use serde::Serialize;
use study_core::catalog::Material;

use crate::catalog::CatalogItem;

use super::references::{DisciplineDto, TeacherDto};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDto {
    id: i64,
    title: String,
    description: String,

    #[serde(rename = "type")]
    material_type: &'static str,

    discipline_id: i64,
    teacher_id: i64,
    tags: Vec<String>,
    year: i32,
    course: u8,
    created_at: String,
    date_label: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,
}

impl From<Material> for MaterialDto {
    fn from(value: Material) -> Self {
        Self {
            id: value.id.0,
            title: value.title,
            description: value.description,
            material_type: value.material_type.as_db_value(),
            discipline_id: value.discipline_id.0,
            teacher_id: value.teacher_id.0,
            tags: value.tags,
            year: value.year,
            course: value.course,
            created_at: value.created_at,
            date_label: "Добавлено",
            preview: value.preview_url,
            duration: value.duration_seconds.map(format_duration),
            file_size: value.file_size_bytes.map(format_file_size),
            author_name: value.author_name,
        }
    }
}

#[derive(Serialize)]
pub struct CatalogMaterialDto {
    #[serde(flatten)]
    material: MaterialDto,

    discipline: DisciplineDto,
    teacher: TeacherDto,
}

impl From<CatalogItem> for CatalogMaterialDto {
    fn from(value: CatalogItem) -> Self {
        Self {
            material: value.material.into(),
            discipline: value.discipline.into(),
            teacher: value.teacher.into(),
        }
    }
}

fn format_duration(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

fn format_file_size(bytes: i64) -> String {
    let bytes = bytes as f64;

    if bytes < 1024.0 {
        format!("{bytes:.0} Б")
    } else if bytes < 1024.0 * 1024.0 {
        format!("{:.1} КБ", bytes / 1024.0)
    } else if bytes < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} МБ", bytes / (1024.0 * 1024.0))
    } else {
        format!("{:.1} ГБ", bytes / (1024.0 * 1024.0 * 1024.0))
    }
}
