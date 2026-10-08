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
    pub include_archived: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogQueryError {
    #[error("page must be positive")]
    InvalidPage,
    #[error("page size must be 1..100")]
    InvalidPageSize,
    #[error("invalid filter identifier")]
    InvalidIdentifier,
    #[error("too many or invalid filters")]
    InvalidFilters,
}
impl CatalogQuery {
    pub fn validate(&self) -> Result<(), CatalogQueryError> {
        if self.page == 0 {
            return Err(CatalogQueryError::InvalidPage);
        }
        if !(1..=100).contains(&self.page_size) {
            return Err(CatalogQueryError::InvalidPageSize);
        }
        if self.discipline_ids.iter().any(|id| id.0 <= 0)
            || self.teacher_ids.iter().any(|id| id.0 <= 0)
        {
            return Err(CatalogQueryError::InvalidIdentifier);
        }
        if self.text.chars().count() > 256
            || self.material_types.len()
                + self.discipline_ids.len()
                + self.teacher_ids.len()
                + self.courses.len()
                + self.years.len()
                + self.tags.len()
                > 100
            || self.courses.contains(&0)
            || self.years.iter().any(|year| !(1..=9999).contains(year))
            || self
                .tags
                .iter()
                .any(|tag| tag.trim().is_empty() || tag.chars().count() > 80)
        {
            return Err(CatalogQueryError::InvalidFilters);
        }
        Ok(())
    }
}
