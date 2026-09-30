use anyhow::{Context, Result, ensure};
use sqlx::FromRow;
use study_core::catalog::{Material, MaterialType};
use study_core::ids::{DisciplineId, MaterialId, TeacherId};

#[derive(FromRow)]
pub(super) struct MaterialRow {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub material_type: String,
    pub discipline_id: i64,
    pub teacher_id: i64,
    pub course: i64,
    pub year: i32,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub author_name: Option<String>,
    pub preview_url: Option<String>,
    pub duration_seconds: Option<i64>,
    pub file_size_bytes: Option<i64>,
}

impl TryFrom<MaterialRow> for Material {
    type Error = anyhow::Error;

    fn try_from(row: MaterialRow) -> Result<Self> {
        ensure!(
            row.id > 0 && row.discipline_id > 0 && row.teacher_id > 0,
            "invalid material references"
        );
        ensure!(
            row.file_size_bytes.is_none_or(|value| value >= 0),
            "negative file size for material {}",
            row.id
        );
        let material_type = MaterialType::from_db_value(&row.material_type).with_context(|| {
            format!(
                "unknown material type for material {}: {}",
                row.id, row.material_type
            )
        })?;
        let course = u8::try_from(row.course).context("course exceeds u8")?;
        ensure!(course > 0, "zero course for material {}", row.id);
        let duration_seconds = row
            .duration_seconds
            .map(u32::try_from)
            .transpose()
            .context("duration exceeds u32")?;

        Ok(Self {
            id: MaterialId(row.id),
            title: row.title,
            description: row.description,
            material_type,
            discipline_id: DisciplineId(row.discipline_id),
            teacher_id: TeacherId(row.teacher_id),
            tags: Vec::new(),
            course,
            year: row.year,
            created_at: row.created_at,
            updated_at: row.updated_at,
            author_name: row.author_name,
            preview_url: row.preview_url,
            duration_seconds,
            file_size_bytes: row.file_size_bytes,
        })
    }
}
