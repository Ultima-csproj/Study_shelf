use crate::ids::{DisciplineId, InstitutionId};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum LearningSort {
    Newest,
    Oldest,
    Title,
}

#[derive(Debug, Clone, Copy)]
pub struct PageRequest {
    pub page: u32,
    pub page_size: u32,
}
impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

#[derive(Debug, Error)]
pub enum LearningQueryError {
    #[error("page must be positive and page size must be 1..100")]
    InvalidPage,
    #[error("invalid identifier")]
    InvalidIdentifier,
    #[error("invalid or excessive filters")]
    InvalidFilters,
}
impl PageRequest {
    pub fn validate(self) -> Result<(), LearningQueryError> {
        if self.page == 0 || !(1..=100).contains(&self.page_size) {
            return Err(LearningQueryError::InvalidPage);
        }
        Ok(())
    }
    pub fn offset(self) -> Result<i64, LearningQueryError> {
        self.validate()?;
        Ok(i64::from(self.page - 1) * i64::from(self.page_size))
    }
}

#[derive(Debug, Clone)]
pub struct LearningQuery {
    pub text: String,
    pub discipline_ids: Vec<DisciplineId>,
    pub institution_id: Option<InstitutionId>,
    pub courses: Vec<u8>,
    pub sort: LearningSort,
    pub pagination: PageRequest,
}
impl Default for LearningQuery {
    fn default() -> Self {
        Self {
            text: String::new(),
            discipline_ids: Vec::new(),
            institution_id: None,
            courses: Vec::new(),
            sort: LearningSort::Title,
            pagination: PageRequest::default(),
        }
    }
}
impl LearningQuery {
    pub fn validate(&self) -> Result<(), LearningQueryError> {
        self.pagination.validate()?;
        if self.discipline_ids.iter().any(|id| id.0 <= 0)
            || self.institution_id.is_some_and(|id| id.0 <= 0)
        {
            return Err(LearningQueryError::InvalidIdentifier);
        }
        if self.text.chars().count() > 256
            || self.courses.contains(&0)
            || self.discipline_ids.len() + self.courses.len() > 100
        {
            return Err(LearningQueryError::InvalidFilters);
        }
        Ok(())
    }
}
