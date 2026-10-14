use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::PgConnection;
use study_core::catalog::DisciplineReference;
use study_core::error::RepositoryResult;
use study_core::ids::DisciplineId;

pub(super) async fn load(
    conn: &mut PgConnection,
) -> RepositoryResult<(Vec<DisciplineReference>, Vec<u8>)> {
    let mut disciplines = Vec::new();
    {
        let mut rows = sqlx::query_as::<_, (i64, String, String)>(
            r#"
                SELECT DISTINCT
                  d.id,
                  d.slug,
                  d.name
                FROM
                  collections c
                  JOIN disciplines d ON d.id = c.discipline_id
                WHERE
                  c.visibility = 'public'
                ORDER BY
                  d.name,
                  d.id
            "#,
        )
        .fetch(&mut *conn);
        while let Some((id, slug, name)) = rows.try_next().await.map_err(map_sqlx_error)? {
            disciplines.push(DisciplineReference {
                id: DisciplineId(id),
                slug,
                name,
            });
        }
    }
    let mut courses = Vec::new();
    let mut rows = sqlx::query_scalar::<_, i16>(
        r#"
            SELECT DISTINCT
              course
            FROM
              collections
            WHERE
              visibility = 'public'
              AND course IS NOT NULL
            ORDER BY
              course
        "#,
    )
    .fetch(conn);
    while let Some(value) = rows.try_next().await.map_err(map_sqlx_error)? {
        courses.push(crate::learning::course(value)?);
    }
    Ok((disciplines, courses))
}
