use sqlx::FromRow;
use study_core::catalog::{Material, MaterialType};
use study_core::error::{RepositoryError, RepositoryResult};
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
    type Error = RepositoryError;

    fn try_from(row: MaterialRow) -> RepositoryResult<Self> {
        if row.id <= 0 || row.discipline_id <= 0 || row.teacher_id <= 0 {
            return Err(RepositoryError::invalid_data(format!(
                "invalid references for material {}",
                row.id,
            )));
        }
        if row.file_size_bytes.is_some_and(|value| value < 0) {
            return Err(RepositoryError::invalid_data(format!(
                "negative file size for material {}",
                row.id,
            )));
        }
        let material_type = MaterialType::from_db_value(&row.material_type).ok_or_else(|| {
            RepositoryError::invalid_data(format!(
                "unknown material type for material {}: {}",
                row.id, row.material_type
            ))
        })?;
        let course = u8::try_from(row.course).map_err(|error| {
            RepositoryError::invalid_data_with_source(
                format!("course exceeds u8 for material {}", row.id),
                error,
            )
        })?;
        if course == 0 {
            return Err(RepositoryError::invalid_data(format!(
                "zero course for material {}",
                row.id,
            )));
        }
        let duration_seconds = row
            .duration_seconds
            .map(u32::try_from)
            .transpose()
            .map_err(|error| {
                RepositoryError::invalid_data_with_source(
                    format!("duration exceeds u32 for material {}", row.id),
                    error,
                )
            })?;

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
