use std::sync::Arc;

use crate::catalog::CatalogService;

#[derive(Clone)]
pub struct AppState {
    pub catalog: Arc<CatalogService>,
}

impl AppState {
    pub fn new(catalog: CatalogService) -> Self {
        Self {
            catalog: Arc::new(catalog),
        }
    }
}
