use super::{children, row::SummaryRow};
use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::PgConnection;
use study_core::catalog::MaterialSummary;
use study_core::error::RepositoryResult;

pub(crate) async fn load_public(
    conn: &mut PgConnection,
    ids: &[i64],
) -> RepositoryResult<Vec<MaterialSummary>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut result = Vec::with_capacity(ids.len());
    {
        let mut rows = sqlx::query_as::<_, SummaryRow>(
            r#"
                SELECT
                  *
                FROM
                  public_materials
                WHERE
                  id = ANY ($1)
                ORDER BY
                  id
            "#,
        )
        .bind(ids)
        .fetch(&mut *conn);
        while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
            result.push(row.into_summary()?);
        }
    }
    children::populate(conn, &mut result).await?;
    Ok(result)
}
