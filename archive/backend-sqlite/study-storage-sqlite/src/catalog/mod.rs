mod facets;
mod reference_row;
mod references;
mod row;
mod search;

use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use study_core::error::RepositoryResult as Result;

use study_core::catalog::{
    CatalogFacets, CatalogPage, CatalogQuery, CatalogRepository, DisciplineReference, Material,
    TeacherReference,
};

use study_core::ids::{DisciplineId, MaterialId, TeacherId};

use row::MaterialRow;

#[derive(Clone)]
pub struct SqliteCatalogRepository {
    pool: SqlitePool,
}

impl SqliteCatalogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatalogRepository for SqliteCatalogRepository {
    async fn get_by_id(&self, id: MaterialId) -> Result<Option<Material>> {
        let row: Option<MaterialRow> = sqlx::query_as("SELECT * FROM materials WHERE id = ?")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(crate::error::map_sqlx_error)?;

        let Some(row) = row else {
            return Ok(None);
        };

        let material: Material = row.try_into()?;
        let mut items = vec![material];

        load_tags(&self.pool, &mut items).await?;

        Ok(items.pop())
    }

    async fn search(&self, query: CatalogQuery) -> Result<CatalogPage> {
        search::run(&self.pool, query).await
    }

    async fn facets(&self) -> Result<CatalogFacets> {
        facets::load(&self.pool).await
    }

    async fn disciplines(&self, ids: &[DisciplineId]) -> Result<Vec<DisciplineReference>> {
        references::load_disciplines(&self.pool, ids).await
    }

    async fn teachers(&self, ids: &[TeacherId]) -> Result<Vec<TeacherReference>> {
        references::load_teachers(&self.pool, ids).await
    }
}

async fn load_tags(pool: &SqlitePool, items: &mut [Material]) -> Result<()> {
    if items.is_empty() {
        return Ok(());
    }

    let positions: HashMap<i64, usize> = items
        .iter()
        .enumerate()
        .map(|(index, material)| (material.id.0, index))
        .collect();

    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT material_id, tag
         FROM material_tags
         WHERE material_id IN (",
    );

    {
        let mut separated = builder.separated(", ");

        for material in items.iter() {
            separated.push_bind(material.id.0);
        }

        separated.push_unseparated(") ORDER BY material_id, tag");
    }

    let tags: Vec<(i64, String)> = builder
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(crate::error::map_sqlx_error)?;

    for (id, tag) in tags {
        if let Some(&index) = positions.get(&id) {
            items[index].tags.push(tag);
        }
    }

    Ok(())
}
