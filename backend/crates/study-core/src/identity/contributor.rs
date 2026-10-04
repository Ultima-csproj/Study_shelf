use crate::ids::{ContributorId, InstitutionId, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContributorKind {
    Person {
        user_id: Option<UserId>,
    },
    Organization {
        institution_id: Option<InstitutionId>,
    },
}

/// A creditable identity. Optional links must be confirmed by the application.
#[derive(Debug, Clone)]
pub struct Contributor {
    pub id: ContributorId,
    pub display_name: String,
    pub kind: ContributorKind,
}
