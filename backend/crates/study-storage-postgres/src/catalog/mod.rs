mod children;
mod facets;
pub(crate) mod projection;
mod query;
mod rebuild;
mod row;

use crate::error::map_sqlx_error;
use async_trait::async_trait;
use futures_util::TryStreamExt;
use sqlx::PgPool;
use study_core::catalog::{CatalogPage, CatalogQuery, CatalogRepository, MaterialSummary};
use study_core::error::{RepositoryError, RepositoryResult};
use study_core::ids::MaterialId;

#[derive(Clone)]
pub struct PostgresCatalogRepository {
    pool: PgPool,
}
impl PostgresCatalogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatalogRepository for PostgresCatalogRepository {
    async fn search(&self, query: &CatalogQuery) -> RepositoryResult<CatalogPage> {
        query.validate().map_err(RepositoryError::invalid_query)?;
        let filter = query::PreparedFilter::new(query);
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let total: i64 = filter
            .bind(sqlx::query_scalar(query::count()))
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let total = u64::try_from(total)
            .map_err(|error| RepositoryError::invalid_data_with_source("negative count", error))?;
        let mut items = Vec::with_capacity(query.page_size as usize);
        {
            let offset = i64::from(query.page - 1) * i64::from(query.page_size);
            let mut rows = filter
                .bind_as(sqlx::query_as::<_, row::SummaryRow>(query::select(
                    query.sort,
                )))
                .bind(i64::from(query.page_size))
                .bind(offset)
                .fetch(&mut *tx);
            while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
                items.push(row.into_summary()?);
            }
        }
        children::populate(&mut tx, &mut items).await?;
        let facets = facets::load(&mut tx).await?;
        let page_count =
            u32::try_from(total.div_ceil(u64::from(query.page_size))).map_err(|error| {
                RepositoryError::invalid_data_with_source("page count overflow", error)
            })?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(CatalogPage {
            items,
            total,
            page: query.page,
            page_count,
            facets,
        })
    }
    async fn get_by_id(&self, id: MaterialId) -> RepositoryResult<Option<MaterialSummary>> {
        if id.0 <= 0 {
            return Err(RepositoryError::invalid_query(
                study_core::catalog::CatalogQueryError::InvalidIdentifier,
            ));
        }
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let row =
            sqlx::query_as::<_, row::SummaryRow>("SELECT * FROM public_materials WHERE id=$1")
                .bind(id.0)
                .fetch_optional(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
        let mut items = Vec::with_capacity(1);
        if let Some(row) = row {
            items.push(row.into_summary()?);
        }
        children::populate(&mut tx, &mut items).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(items.pop())
    }
}
