#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("catalog storage failed")]
    Storage(#[from] study_core::error::RepositoryError),
}
