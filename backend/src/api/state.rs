use std::sync::Arc;

use crate::catalog::CatalogService;

#[derive(Clone)]
pub struct AppState {
    pub materials: Arc<study_storage_postgres::material::PostgresMaterialRepository>,
    pub administration: Arc<study_storage_postgres::administration::Administration>,
    pub auth: Arc<crate::auth::AuthService>,
    pub catalog: Arc<CatalogService>,
    pub disciplines: Arc<dyn study_core::discipline::DisciplineRepository>,
    pub collections: Arc<dyn study_core::collection::CollectionRepository>,
}

impl AppState {
    pub fn new(
        materials: study_storage_postgres::material::PostgresMaterialRepository,
        administration: study_storage_postgres::administration::Administration,
        auth: Arc<crate::auth::AuthService>,
        catalog: CatalogService,
        disciplines: impl study_core::discipline::DisciplineRepository + 'static,
        collections: impl study_core::collection::CollectionRepository + 'static,
    ) -> Self {
        Self {
            materials: Arc::new(materials),
            administration: Arc::new(administration),
            auth,
            catalog: Arc::new(catalog),
            disciplines: Arc::new(disciplines),
            collections: Arc::new(collections),
        }
    }
}
