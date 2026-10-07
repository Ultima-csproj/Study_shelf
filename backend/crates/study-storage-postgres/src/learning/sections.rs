use super::{ScopeSql, section_queries, section_rows::EntryRow};
use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::{FromRow, PgConnection};
use std::collections::{HashMap, HashSet};
use study_core::catalog::MaterialSummary;
use study_core::error::RepositoryResult;
use study_core::ids::SectionId;
use study_core::learning::{LearningSection, PageRequest, SectionPage, SectionScope};

const INITIAL_PAGE_SIZE: i64 = 10;
#[derive(FromRow)]
struct SectionRow {
    id: i64,
    title: String,
    description: String,
    position: i64,
    step_count: i64,
}

pub(crate) async fn detail_sections(
    conn: &mut PgConnection,
    scope: SectionScope,
) -> RepositoryResult<(Vec<LearningSection>, Vec<MaterialSummary>)> {
    let queries = section_queries::get(scope);
    let parent = ScopeSql::new(scope).parent;
    let mut sections = Vec::new();
    let mut positions = HashMap::new();
    {
        let mut rows = sqlx::query_as::<_, SectionRow>(queries.headers.as_str())
            .bind(parent)
            .fetch(&mut *conn);
        while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
            if row.id <= 0 || row.position <= 0 {
                return Err(crate::error::invalid_relation("section identity"));
            }
            positions.insert(row.id, sections.len());
            sections.push(LearningSection {
                id: SectionId(row.id),
                title: row.title,
                description: row.description,
                position: super::count(row.position)?,
                step_count: super::count(row.step_count)?,
                items: Vec::with_capacity(INITIAL_PAGE_SIZE as usize),
            });
        }
    }
    let mut ids = HashSet::new();
    {
        let mut rows = sqlx::query_as::<_, EntryRow>(queries.initial.as_str())
            .bind(parent)
            .bind(INITIAL_PAGE_SIZE)
            .fetch(&mut *conn);
        while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
            let index = *positions
                .get(&row.section_id)
                .ok_or_else(|| crate::error::invalid_relation("section parent"))?;
            let entry = row.into_entry()?;
            ids.insert(entry.material_id.0);
            sections[index].items.push(entry);
        }
    }
    let ids: Vec<i64> = ids.into_iter().collect();
    let materials = crate::catalog::batch::load_public(conn, &ids).await?;
    Ok((sections, materials))
}
pub(crate) async fn section_items(
    conn: &mut PgConnection,
    scope: SectionScope,
    section: SectionId,
    request: PageRequest,
) -> RepositoryResult<Option<SectionPage>> {
    let queries = section_queries::get(scope);
    let parent = ScopeSql::new(scope).parent;
    let exists: bool = sqlx::query_scalar(queries.exists.as_str())
        .bind(section.0)
        .bind(parent)
        .fetch_one(&mut *conn)
        .await
        .map_err(map_sqlx_error)?;
    if !exists {
        return Ok(None);
    }
    let total: i64 = sqlx::query_scalar(queries.count.as_str())
        .bind(parent)
        .bind(section.0)
        .fetch_one(&mut *conn)
        .await
        .map_err(map_sqlx_error)?;
    let mut entries = Vec::with_capacity(request.page_size as usize);
    let mut ids = HashSet::with_capacity(request.page_size as usize);
    {
        let mut rows = sqlx::query_as::<_, EntryRow>(queries.page.as_str())
            .bind(parent)
            .bind(section.0)
            .bind(i64::from(request.page_size))
            .bind(
                request
                    .offset()
                    .map_err(study_core::error::RepositoryError::invalid_query)?,
            )
            .fetch(&mut *conn);
        while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
            let entry = row.into_entry()?;
            ids.insert(entry.material_id.0);
            entries.push(entry);
        }
    }
    let ids: Vec<i64> = ids.into_iter().collect();
    let materials = crate::catalog::batch::load_public(conn, &ids).await?;
    Ok(Some(SectionPage {
        items: super::page(entries, total, request)?,
        materials,
    }))
}
