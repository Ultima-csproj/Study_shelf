use study_core::catalog::{CatalogFacets, DisciplineReference, Material, TeacherReference};

pub struct CatalogItem {
    pub material: Material,
    pub discipline: DisciplineReference,
    pub teacher: TeacherReference,
}

pub struct CatalogView {
    pub items: Vec<CatalogItem>,
    pub total: u64,
    pub page: u32,
    pub page_count: u32,
    pub facets: CatalogFacets,
}
