use crate::ids::{DisciplineId, TeacherId};

use super::MaterialType;

#[derive(Debug, Clone, Copy)]
pub enum CatalogSort {
    Newest,
    Oldest,
    Title,
}

#[derive(Debug, Clone)]
pub struct CatalogQuery {
    pub text: String,
    pub material_types: Vec<MaterialType>,
    pub discipline_ids: Vec<DisciplineId>,
    pub teacher_ids: Vec<TeacherId>,
    pub courses: Vec<u8>,
    pub years: Vec<i32>,
    pub tags: Vec<String>,
    pub sort: CatalogSort,
    pub page: u32,
    pub page_size: u32,
}
