use std::error::Error;
use std::fmt;

use study_core::error::RepositoryError;
use study_core::ids::{DisciplineId, MaterialId, TeacherId};

#[derive(Debug)]
pub enum CatalogError {
    Storage(RepositoryError),
    MissingDiscipline {
        material_id: MaterialId,
        discipline_id: DisciplineId,
    },
    MissingTeacher {
        material_id: MaterialId,
        teacher_id: TeacherId,
    },
}

impl From<RepositoryError> for CatalogError {
    fn from(error: RepositoryError) -> Self {
        Self::Storage(error)
    }
}

impl fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(error) => write!(formatter, "catalog storage: {error}"),
            Self::MissingDiscipline {
                material_id,
                discipline_id,
            } => write!(
                formatter,
                "material {} references missing discipline {}",
                material_id.0, discipline_id.0,
            ),
            Self::MissingTeacher {
                material_id,
                teacher_id,
            } => write!(
                formatter,
                "material {} references missing teacher {}",
                material_id.0, teacher_id.0,
            ),
        }
    }
}

impl Error for CatalogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Storage(error) => Some(error),
            Self::MissingDiscipline { .. } | Self::MissingTeacher { .. } => None,
        }
    }
}
