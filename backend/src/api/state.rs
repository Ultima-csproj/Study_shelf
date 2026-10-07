use std::sync::Arc;

use crate::catalog::CatalogService;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<crate::auth::AuthService>,
    pub catalog: Arc<CatalogService>,
    pub disciplines: Arc<dyn study_core::discipline::DisciplineRepository>,
    pub collections: Arc<dyn study_core::collection::CollectionRepository>,
}

impl AppState {
    pub fn new(
        auth: Arc<crate::auth::AuthService>,
        catalog: CatalogService,
        disciplines: impl study_core::discipline::DisciplineRepository + 'static,
        collections: impl study_core::collection::CollectionRepository + 'static,
    ) -> Self {
        Self {
            auth,
            catalog: Arc::new(catalog),
            disciplines: Arc::new(disciplines),
            collections: Arc::new(collections),
        }
    }
}
