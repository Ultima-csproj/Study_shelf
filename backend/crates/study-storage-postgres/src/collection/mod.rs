mod facets;
mod participants;
mod query;
mod row;
use crate::error::map_sqlx_error;
use async_trait::async_trait;
use futures_util::TryStreamExt;
use sqlx::{PgConnection, PgPool};
use study_core::catalog::normalize_search_text;
use study_core::collection::{CollectionPage, CollectionRepository, CollectionSummary};
use study_core::error::{RepositoryError, RepositoryResult};
use study_core::ids::{CollectionId, SectionId};
use study_core::learning::{LearningDetail, LearningQuery, PageRequest, SectionPage, SectionScope};

#[derive(Clone)]
pub struct PostgresCollectionRepository {
    pool: PgPool,
}
impl PostgresCollectionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
async fn one(
    conn: &mut PgConnection,
    id: CollectionId,
) -> RepositoryResult<Option<CollectionSummary>> {
    let row = sqlx::query_as::<_, row::CollectionRow>(&**query::ONE)
        .bind(id.0)
        .fetch_optional(&mut *conn)
        .await
        .map_err(map_sqlx_error)?;
    let Some(row) = row else {
        return Ok(None);
    };
    let mut items = vec![row.into_summary()?];
    participants::populate(conn, &mut items).await?;
    Ok(items.pop())
}
#[async_trait]
impl CollectionRepository for PostgresCollectionRepository {
    async fn list(&self, query: &LearningQuery) -> RepositoryResult<CollectionPage> {
        query.validate().map_err(RepositoryError::invalid_query)?;
        let text = normalize_search_text(&query.text);
        let disciplines: Vec<i64> = query.discipline_ids.iter().map(|id| id.0).collect();
        let courses: Vec<i16> = query
            .courses
            .iter()
            .map(|course| i16::from(*course))
            .collect();
        let institution = query.institution_id.map(|id| id.0);
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let total: i64 = sqlx::query_scalar(&**query::COUNT)
            .bind(&text)
            .bind(&disciplines)
            .bind(&courses)
            .bind(institution)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let mut items = Vec::with_capacity(query.pagination.page_size as usize);
        {
            let mut rows = sqlx::query_as::<_, row::CollectionRow>(query::select(query.sort))
                .bind(&text)
                .bind(&disciplines)
                .bind(&courses)
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
        participants::populate(&mut tx, &mut items).await?;
        let (disciplines, courses) = facets::load(&mut tx).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(CollectionPage {
            page: crate::learning::page(items, total, query.pagination)?,
            disciplines,
            courses,
        })
    }
    async fn get_by_id(
        &self,
        id: CollectionId,
    ) -> RepositoryResult<Option<LearningDetail<CollectionSummary>>> {
        crate::learning::validate_id(id.0)?;
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let Some(summary) = one(&mut tx, id).await? else {
            return Ok(None);
        };
        let (sections, materials) =
            crate::learning::detail_sections(&mut tx, SectionScope::Collection(id)).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(Some(LearningDetail {
            summary,
            sections,
            materials,
        }))
    }
    async fn section_items(
        &self,
        id: CollectionId,
        section: SectionId,
        page: PageRequest,
    ) -> RepositoryResult<Option<SectionPage>> {
        crate::learning::validate_id(id.0)?;
        crate::learning::validate_id(section.0)?;
        page.validate().map_err(RepositoryError::invalid_query)?;
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let visible: bool = sqlx::query_scalar(
            r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      collections
                    WHERE
                      id = $1
                      AND visibility = 'public'
                  )
            "#,
        )
        .bind(id.0)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if !visible {
            return Ok(None);
        }
        let result =
            crate::learning::section_items(&mut tx, SectionScope::Collection(id), section, page)
                .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(result)
    }
}
