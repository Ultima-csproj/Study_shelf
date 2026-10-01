use anyhow::Result;
use async_trait::async_trait;

use crate::ids::{DisciplineId, MaterialId, TeacherId};

use super::{
    CatalogFacets, CatalogPage, CatalogQuery, DisciplineReference, Material, TeacherReference,
};

#[async_trait]
pub trait CatalogRepository: Send + Sync {
    async fn get_by_id(&self, id: MaterialId) -> Result<Option<Material>>;

    async fn search(&self, query: CatalogQuery) -> Result<CatalogPage>;

    async fn facets(&self) -> Result<CatalogFacets>;

    async fn disciplines(&self, ids: &[DisciplineId]) -> Result<Vec<DisciplineReference>>;

    async fn teachers(&self, ids: &[TeacherId]) -> Result<Vec<TeacherReference>>;
}
