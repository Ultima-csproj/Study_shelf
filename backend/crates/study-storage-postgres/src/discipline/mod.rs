mod responsible;
mod row;
use crate::error::map_sqlx_error;
use crate::learning::{detail_sections, section_items};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use sqlx::{PgConnection, PgPool};
use std::sync::LazyLock;
use study_core::catalog::normalize_search_text;
use study_core::discipline::{DisciplineRepository, DisciplineSummary};
use study_core::error::{RepositoryError, RepositoryResult};
use study_core::ids::{DisciplineId, SectionId};
use study_core::learning::{
    LearningDetail, LearningPage, LearningQuery, PageRequest, SectionPage, SectionScope,
};

const SUMMARY: &str = include_str!("sql/summary.sql");
const FILTER: &str = include_str!("sql/filter.sql");
static SELECT: LazyLock<String> =
    LazyLock::new(|| format!("{SUMMARY} {FILTER} ORDER BY d.name,d.id LIMIT $3 OFFSET $4"));
static ONE: LazyLock<String> = LazyLock::new(|| format!("{SUMMARY} WHERE d.id=$1"));
static COUNT: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
        SELECT
          count(*)
        FROM
          disciplines d {FILTER}
    "#
    )
});

#[derive(Clone)]
pub struct PostgresDisciplineRepository {
    pool: PgPool,
}
impl PostgresDisciplineRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
pub(super) async fn one(
    conn: &mut PgConnection,
    id: DisciplineId,
) -> RepositoryResult<Option<DisciplineSummary>> {
    let row = sqlx::query_as::<_, row::DisciplineRow>(&**ONE)
        .bind(id.0)
        .fetch_optional(&mut *conn)
        .await
        .map_err(map_sqlx_error)?;
    let Some(row) = row else {
        return Ok(None);
    };
    let mut items = vec![row.into_summary()?];
    responsible::populate(conn, &mut items).await?;
    Ok(items.pop())
}
#[async_trait]
impl DisciplineRepository for PostgresDisciplineRepository {
    async fn list(
        &self,
        query: &LearningQuery,
    ) -> RepositoryResult<LearningPage<DisciplineSummary>> {
        query.validate().map_err(RepositoryError::invalid_query)?;
        let text = normalize_search_text(&query.text);
        let institution = query.institution_id.map(|id| id.0);
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let total: i64 = sqlx::query_scalar(&**COUNT)
            .bind(&text)
            .bind(institution)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let mut items = Vec::with_capacity(query.pagination.page_size as usize);
        {
            let mut rows = sqlx::query_as::<_, row::DisciplineRow>(&**SELECT)
                .bind(&text)
                .bind(institution)
                .bind(i64::from(query.pagination.page_size))
                .bind(
                    query
                        .pagination
                        .offset()
                        .map_err(RepositoryError::invalid_query)?,
                )
                .fetch(&mut *tx);
            while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
                items.push(row.into_summary()?);
            }
        }
        responsible::populate(&mut tx, &mut items).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        crate::learning::page(items, total, query.pagination)
    }
    async fn get_by_id(
        &self,
        id: DisciplineId,
    ) -> RepositoryResult<Option<LearningDetail<DisciplineSummary>>> {
        crate::learning::validate_id(id.0)?;
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let Some(summary) = one(&mut tx, id).await? else {
            return Ok(None);
        };
        let (sections, materials) = detail_sections(&mut tx, SectionScope::Discipline(id)).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(Some(LearningDetail {
            summary,
            sections,
            materials,
        }))
    }
    async fn section_items(
        &self,
        id: DisciplineId,
        section: SectionId,
        page: PageRequest,
    ) -> RepositoryResult<Option<SectionPage>> {
        crate::learning::validate_id(id.0)?;
        crate::learning::validate_id(section.0)?;
        page.validate().map_err(RepositoryError::invalid_query)?;
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let result = section_items(&mut tx, SectionScope::Discipline(id), section, page).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(result)
    }
}
