mod error;
pub use error::CatalogError;
use std::sync::Arc;
use study_core::catalog::{CatalogPage, CatalogQuery, CatalogRepository, MaterialSummary};
use study_core::ids::MaterialId;
pub type CatalogItem = MaterialSummary;
pub type CatalogView = CatalogPage;

pub struct CatalogService {
    repository: Arc<dyn CatalogRepository>,
}
impl CatalogService {
    pub fn new<R: CatalogRepository + 'static>(repository: R) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
    pub async fn list(&self, query: CatalogQuery) -> Result<CatalogView, CatalogError> {
        Ok(self.repository.search(&query).await?)
    }
    pub async fn get_by_id(&self, id: MaterialId) -> Result<Option<CatalogItem>, CatalogError> {
        Ok(self.repository.get_by_id(id).await?)
    }
}
