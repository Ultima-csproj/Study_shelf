mod membership;

use crate::ids::InstitutionId;

pub use membership::{AcademicRole, InstitutionMembership, InstitutionPermission, MembershipState};

/// An approved institution. Pending registration requests are a separate future model.
#[derive(Debug, Clone)]
pub struct Institution {
    pub id: InstitutionId,
    pub name: String,
}
