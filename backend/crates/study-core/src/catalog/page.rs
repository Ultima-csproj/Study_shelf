use super::Material;

#[derive(Debug, Clone)]
pub struct CatalogPage {
    pub items: Vec<Material>,
    pub total: u64,
    pub page: u32,
    pub page_count: u32,
}
