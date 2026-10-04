use super::{CatalogPage, CatalogQuery, MaterialSummary};
use crate::error::RepositoryResult;
use crate::ids::MaterialId;
use async_trait::async_trait;

#[async_trait]
pub trait CatalogRepository: Send + Sync {
    /// Includes items, count and facets from one consistent read snapshot.
    async fn search(&self, query: &CatalogQuery) -> RepositoryResult<CatalogPage>;
    async fn get_by_id(&self, id: MaterialId) -> RepositoryResult<Option<MaterialSummary>>;
}
