use std::collections::{BTreeMap, HashMap};

use anyhow::{Context, Result};
use sqlx::{FromRow, SqlitePool};
use study_core::catalog::MaterialType;

#[derive(Debug, Clone, FromRow)]
pub struct TeacherFacet {
    pub id: i64,
    pub name: String,
    pub short_name: String,
    pub material_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct DisciplineFacet {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub summary: Option<String>,
    pub course: i64,
    pub semester: i64,
    pub teacher_id: i64,
    pub material_count: i64,
}

#[derive(Debug, Clone)]
pub struct ValueFacet {
    pub value: i64,
    pub material_count: i64,
}

#[derive(Debug, Clone)]
pub struct TypeFacet {
    pub material_type: MaterialType,
    pub material_count: i64,
}

#[derive(Debug, Clone)]
pub struct CatalogFacets {
    pub types: Vec<TypeFacet>,
    pub disciplines: Vec<DisciplineFacet>,
    pub teachers: Vec<TeacherFacet>,
    pub courses: Vec<ValueFacet>,
    pub years: Vec<ValueFacet>,
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
        "SELECT material_type AS value, COUNT(*) AS material_count FROM materials GROUP BY material_type",
    )
    .fetch_all(pool)
    .await?;
    let mut type_counts: HashMap<String, i64> = HashMap::new();
    for row in type_rows {
        MaterialType::from_db_value(&row.value)
            .with_context(|| format!("unknown material type in facets: {}", row.value))?;
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

    let disciplines: Vec<DisciplineFacet> = sqlx::query_as(
        "SELECT d.*, COUNT(m.id) AS material_count FROM disciplines d \
         LEFT JOIN materials m ON m.discipline_id = d.id GROUP BY d.id ORDER BY d.id",
    )
    .fetch_all(pool)
    .await?;
    let teachers: Vec<TeacherFacet> = sqlx::query_as(
        "SELECT t.*, COUNT(m.id) AS material_count FROM teachers t \
         LEFT JOIN materials m ON m.teacher_id = t.id GROUP BY t.id ORDER BY t.id",
    )
    .fetch_all(pool)
    .await?;
    let course_rows: Vec<NumberCountRow> = sqlx::query_as(
        "SELECT course AS value, COUNT(*) AS material_count FROM materials GROUP BY course ORDER BY course",
    )
    .fetch_all(pool)
    .await?;
    let year_rows: Vec<NumberCountRow> = sqlx::query_as(
        "SELECT year AS value, COUNT(*) AS material_count FROM materials GROUP BY year ORDER BY year DESC",
    )
    .fetch_all(pool)
    .await?;

    let mut course_counts: BTreeMap<i64, i64> = (1..=4).map(|course| (course, 0)).collect();
    for row in course_rows {
        course_counts.insert(row.value, row.material_count);
    }
    let mut year_counts: BTreeMap<i64, i64> = (2023..=2026).map(|year| (year, 0)).collect();
    for row in year_rows {
        year_counts.insert(row.value, row.material_count);
    }

    Ok(CatalogFacets {
        types,
        disciplines,
        teachers,
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
