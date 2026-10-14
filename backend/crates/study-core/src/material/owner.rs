use crate::ids::{InstitutionId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialOwner {
    User(UserId),
    Institution(InstitutionId),
}
