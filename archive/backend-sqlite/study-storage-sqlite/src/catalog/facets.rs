use std::collections::{BTreeMap, HashMap};

use sqlx::{FromRow, SqlitePool};
use study_core::error::{RepositoryError, RepositoryResult as Result};

use study_core::catalog::{
    CatalogFacets, DisciplineFacet, MaterialType, TeacherFacet, TypeFacet, ValueFacet,
};

use super::reference_row::{DisciplineRow, TeacherRow};

#[derive(FromRow)]
struct TeacherFacetRow {
    #[sqlx(flatten)]
    teacher: TeacherRow,

    material_count: i64,
}

impl From<TeacherFacetRow> for TeacherFacet {
    fn from(row: TeacherFacetRow) -> Self {
        Self {
            teacher: row.teacher.into(),
            material_count: row.material_count,
        }
    }
}

#[derive(FromRow)]
struct DisciplineFacetRow {
    #[sqlx(flatten)]
    discipline: DisciplineRow,

    material_count: i64,
}

impl From<DisciplineFacetRow> for DisciplineFacet {
    fn from(row: DisciplineFacetRow) -> Self {
        Self {
            discipline: row.discipline.into(),
            material_count: row.material_count,
        }
    }
}

#[derive(FromRow)]
struct CountRow {
    value: String,
    material_count: i64,
}

#[derive(FromRow)]
struct NumberCountRow {
    value: i64,
    material_count: i64,
}

pub(super) async fn load(pool: &SqlitePool) -> Result<CatalogFacets> {
    let type_rows: Vec<CountRow> = sqlx::query_as(
        "SELECT
            material_type AS value,
            COUNT(*) AS material_count
         FROM materials
         GROUP BY material_type",
    )
    .fetch_all(pool)
    .await
    .map_err(crate::error::map_sqlx_error)?;

    let mut type_counts = HashMap::new();

    for row in type_rows {
        MaterialType::from_db_value(&row.value).ok_or_else(|| {
            RepositoryError::invalid_data(format!("unknown material type in facets: {}", row.value))
        })?;

        type_counts.insert(row.value, row.material_count);
    }

    let types = [
        MaterialType::Lecture,
        MaterialType::Book,
        MaterialType::Article,
        MaterialType::Video,
        MaterialType::Lab,
    ]
    .into_iter()
    .map(|material_type| TypeFacet {
        material_type,
        material_count: *type_counts.get(material_type.as_db_value()).unwrap_or(&0),
    })
    .collect();

    let discipline_rows: Vec<DisciplineFacetRow> = sqlx::query_as(
        "SELECT
             d.*,
             COUNT(m.id) AS material_count
         FROM disciplines d
         LEFT JOIN materials m ON m.discipline_id = d.id
         GROUP BY d.id
         ORDER BY d.id",
    )
    .fetch_all(pool)
    .await
    .map_err(crate::error::map_sqlx_error)?;

    let teacher_rows: Vec<TeacherFacetRow> = sqlx::query_as(
        "SELECT
             t.*,
             COUNT(m.id) AS material_count
         FROM teachers t
         LEFT JOIN materials m ON m.teacher_id = t.id
         GROUP BY t.id
         ORDER BY t.id",
    )
    .fetch_all(pool)
    .await
    .map_err(crate::error::map_sqlx_error)?;

    let course_rows: Vec<NumberCountRow> = sqlx::query_as(
        "SELECT
             course AS value,
             COUNT(*) AS material_count
         FROM materials
         GROUP BY course
         ORDER BY course",
    )
    .fetch_all(pool)
    .await
    .map_err(crate::error::map_sqlx_error)?;

    let year_rows: Vec<NumberCountRow> = sqlx::query_as(
        "SELECT
             year AS value,
             COUNT(*) AS material_count
         FROM materials
         GROUP BY year
         ORDER BY year DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(crate::error::map_sqlx_error)?;

    let mut course_counts: BTreeMap<i64, i64> = (1..=4).map(|value| (value, 0)).collect();

    for row in course_rows {
        course_counts.insert(row.value, row.material_count);
    }

    let mut year_counts: BTreeMap<i64, i64> = (2023..=2026).map(|value| (value, 0)).collect();

    for row in year_rows {
        year_counts.insert(row.value, row.material_count);
    }

    Ok(CatalogFacets {
        types,

        disciplines: discipline_rows.into_iter().map(Into::into).collect(),

        teachers: teacher_rows.into_iter().map(Into::into).collect(),

        courses: course_counts
            .into_iter()
            .map(|(value, material_count)| ValueFacet {
                value,
                material_count,
            })
            .collect(),

        years: year_counts
            .into_iter()
            .rev()
            .map(|(value, material_count)| ValueFacet {
                value,
                material_count,
            })
            .collect(),
    })
}
