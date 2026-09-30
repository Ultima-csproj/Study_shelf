mod facets;
mod row;
mod search;

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use study_core::catalog::{CatalogPage, CatalogQuery, Material, MaterialRepository};
use study_core::ids::MaterialId;

pub use facets::{CatalogFacets, DisciplineFacet, TeacherFacet, TypeFacet, ValueFacet};
use row::MaterialRow;

#[derive(Clone)]
pub struct SqliteCatalogRepository {
    pool: SqlitePool,
}

impl SqliteCatalogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn facets(&self) -> Result<CatalogFacets> {
        facets::load(&self.pool).await
    }
}

#[async_trait]
impl MaterialRepository for SqliteCatalogRepository {
    async fn get_by_id(&self, id: MaterialId) -> Result<Option<Material>> {
        let row: Option<MaterialRow> = sqlx::query_as("SELECT * FROM materials WHERE id = ?")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;
        let Some(row) = row else { return Ok(None) };
        let mut items = vec![row.try_into()?];
        load_tags(&self.pool, &mut items).await?;
        Ok(items.pop())
    }

    async fn search(&self, query: CatalogQuery) -> Result<CatalogPage> {
        search::run(&self.pool, query).await
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
    let mut builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT material_id, tag FROM material_tags WHERE material_id IN (");
    let mut separated = builder.separated(", ");
    for material in items.iter() {
        separated.push_bind(material.id.0);
    }
    separated.push_unseparated(") ORDER BY material_id, tag");
    let tags: Vec<(i64, String)> = builder.build_query_as().fetch_all(pool).await?;
    for (id, tag) in tags {
        if let Some(&index) = positions.get(&id) {
            items[index].tags.push(tag);
        }
    }
    Ok(())
}
