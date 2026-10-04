use super::{CatalogFacets, MaterialSummary};

#[derive(Debug, Clone)]
pub struct CatalogPage {
    pub items: Vec<MaterialSummary>,
    pub total: u64,
    pub page: u32,
    pub page_count: u32,
    pub facets: CatalogFacets,
}
