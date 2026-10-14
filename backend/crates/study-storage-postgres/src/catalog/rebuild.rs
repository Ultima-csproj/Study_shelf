use super::{PostgresCatalogRepository, children, projection, row::SummaryRow};
use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use study_core::error::RepositoryResult;

const BATCH_SIZE: usize = 256;

impl PostgresCatalogRepository {
    /// Explicit maintenance operation. Public requests never rebuild this projection.
    pub async fn rebuild_search(&self) -> RepositoryResult<u64> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        // Prevent pointer/visibility changes while replacing the derived projection.
        sqlx::query("LOCK TABLE materials IN SHARE ROW EXCLUSIVE MODE")
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        sqlx::query("DELETE FROM material_search")
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let mut cursor = 0_i64;
        let mut count = 0_u64;
        loop {
            let mut items = Vec::with_capacity(BATCH_SIZE);
            {
                let mut rows = sqlx::query_as::<_, SummaryRow>(
                    r#"
                        SELECT
                          m.id,
                          m.published_revision_id,
                          m.first_published_at,
                          r.title,
                          r.description,
                          r.material_type,
                          r.source_year,
                          r.preview_url,
                          r.duration_seconds,
                          r.file_size_bytes,
                          r.published_at
                        FROM
                          materials AS m
                          JOIN material_revisions AS r ON r.id = m.published_revision_id
                          AND r.material_id = m.id
                        WHERE
                          r.state = 'published'
                          AND m.id > $1
                        ORDER BY
                          m.id
                        LIMIT
                          $2
                    "#,
                )
                .bind(cursor)
                .bind(BATCH_SIZE as i64)
                .fetch(&mut *tx);
                while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
                    items.push(row.into_summary()?);
                }
            }
            if items.is_empty() {
                break;
            }
            children::populate(&mut tx, &mut items).await?;
            for item in &items {
                projection::upsert(
                    &mut tx,
                    projection::SearchDocument {
                        material_id: item.id,
                        revision_id: item.published_revision_id,
                        title: &item.title,
                        description: &item.description,
                        tags: &item.tags,
                        credits: &item.credits,
                    },
                )
                .await?;
                cursor = item.id.0;
                count += 1;
            }
        }
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(count)
    }
}
