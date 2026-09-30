use anyhow::Result;
use async_trait::async_trait;

use crate::ids::MaterialId;

use super::{CatalogPage, CatalogQuery, Material};

#[async_trait]
pub trait MaterialRepository: Send + Sync {
    async fn get_by_id(&self, id: MaterialId) -> Result<Option<Material>>;

    async fn search(&self, query: CatalogQuery) -> Result<CatalogPage>;
}
 