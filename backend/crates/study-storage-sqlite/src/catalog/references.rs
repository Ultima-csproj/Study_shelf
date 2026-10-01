use anyhow::Result;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use study_core::catalog::{DisciplineReference, TeacherReference};

use study_core::ids::{DisciplineId, TeacherId};

use super::reference_row::{DisciplineRow, TeacherRow};

const IDS_PER_QUERY: usize = 500;

pub(super) async fn load_teachers(
    pool: &SqlitePool,
    ids: &[TeacherId],
) -> Result<Vec<TeacherReference>> {
    let mut result = Vec::new();

    for chunk in ids.chunks(IDS_PER_QUERY) {
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT id, name, short_name
             FROM teachers
             WHERE id IN (",
        );

        {
            let mut separated = builder.separated(", ");

            for id in chunk {
                separated.push_bind(id.0);
            }

            separated.push_unseparated(") ORDER BY id");
        }

        let rows: Vec<TeacherRow> = builder.build_query_as().fetch_all(pool).await?;

        result.extend(rows.into_iter().map(TeacherReference::from));
    }

    Ok(result)
}

pub(super) async fn load_disciplines(
    pool: &SqlitePool,
    ids: &[DisciplineId],
) -> Result<Vec<DisciplineReference>> {
    let mut result = Vec::new();

    for chunk in ids.chunks(IDS_PER_QUERY) {
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT
                id,
                slug,
                name,
                description,
                summary,
                course,
                semester,
                teacher_id
             FROM disciplines
             WHERE id IN (",
        );

        {
            let mut separated = builder.separated(", ");

            for id in chunk {
                separated.push_bind(id.0);
            }

            separated.push_unseparated(") ORDER BY id");
        }

        let rows: Vec<DisciplineRow> = builder.build_query_as().fetch_all(pool).await?;

        result.extend(rows.into_iter().map(DisciplineReference::from));
    }

    Ok(result)
}
