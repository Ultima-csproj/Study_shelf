use std::error::Error;
use std::fmt;

use crate::ids::{MaterialId, MaterialRevisionId};

use super::RevisionState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterialError {
    PermissionDenied,
    InvalidIdentifier(&'static str),
    RevisionIdReused,
    RevisionMaterialMismatch {
        material_id: MaterialId,
        revision_material_id: MaterialId,
    },
    InvalidRevisionState {
        expected: RevisionState,
        actual: RevisionState,
    },
    OutdatedBase {
        base_revision_id: Option<MaterialRevisionId>,
        published_revision_id: Option<MaterialRevisionId>,
    },
    MaterialIsPrivate,
    InvalidContent(super::ContentError),
    InvalidTimestamp,
}

impl fmt::Display for MaterialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionDenied => formatter.write_str("material permission denied"),
            Self::InvalidIdentifier(name) => write!(formatter, "invalid {name} identifier"),
            Self::RevisionIdReused => {
                formatter.write_str("a draft needs a new revision identifier")
            }
            Self::RevisionMaterialMismatch {
                material_id,
                revision_material_id,
            } => write!(
                formatter,
                "material {} does not own revision of material {}",
                material_id.0, revision_material_id.0,
            ),
            Self::InvalidRevisionState { expected, actual } => write!(
                formatter,
                "expected revision state {expected:?}, got {actual:?}",
            ),
            Self::OutdatedBase {
                base_revision_id,
                published_revision_id,
            } => write!(
                formatter,
                "outdated revision base {base_revision_id:?}; current publication {published_revision_id:?}",
            ),
            Self::MaterialIsPrivate => {
                formatter.write_str("cannot publicly publish a private material")
            }
            Self::InvalidContent(error) => fmt::Display::fmt(error, formatter),
            Self::InvalidTimestamp => {
                formatter.write_str("timestamp precedes an earlier operation")
            }
        }
    }
}

impl Error for MaterialError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidContent(error) => Some(error),
            _ => None,
        }
    }
}
