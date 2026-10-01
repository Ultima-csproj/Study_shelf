use async_trait::async_trait;

use crate::error::RepositoryResult;

use crate::ids::{DisciplineId, MaterialId, TeacherId};

use super::{
    CatalogFacets, CatalogPage, CatalogQuery, DisciplineReference, Material, TeacherReference,
};

#[async_trait]
pub trait CatalogRepository: Send + Sync {
    async fn get_by_id(&self, id: MaterialId) -> RepositoryResult<Option<Material>>;

    async fn search(&self, query: CatalogQuery) -> RepositoryResult<CatalogPage>;

    async fn facets(&self) -> RepositoryResult<CatalogFacets>;

    async fn disciplines(&self, ids: &[DisciplineId])
    -> RepositoryResult<Vec<DisciplineReference>>;

    async fn teachers(&self, ids: &[TeacherId]) -> RepositoryResult<Vec<TeacherReference>>;
}
