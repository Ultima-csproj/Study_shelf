use anyhow::Result;
use sqlx::SqlitePool;
use study_core::ids::{DisciplineId, TeacherId};

use super::{DisciplineFacet, TeacherFacet};

pub(super) async fn teacher(pool: &SqlitePool, id: TeacherId) -> Result<Option<TeacherFacet>> {
    let row = sqlx::query_as(
        "SELECT t.*,
            (
                SELECT COUNT(*)
                FROM materials m
                WHERE m.teacher_id = t.id
            ) AS material_count
         FROM teachers t
         WHERE t.id = ?",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub(super) async fn discipline(
    pool: &SqlitePool,
    id: DisciplineId,
) -> Result<Option<DisciplineFacet>> {
    let row = sqlx::query_as(
        "SELECT d.*,
            (
                SELECT COUNT(*)
                FROM materials m
                WHERE m.discipline_id = d.id
            ) AS material_count
         FROM disciplines d
         WHERE d.id = ?",
    )
    .bind(id.0)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
