use crate::ids::{InstitutionId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcademicRole {
    Student,
    Teacher,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipState {
    Pending,
    Active,
    Rejected,
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstitutionPermission {
    ManageMembers,
    ManageDisciplines,
    CreateMaterials,
    ManageMaterials,
    RestrictMaterials,
    ReviewRestrictedMaterials,
}

#[derive(Debug, Clone)]
pub struct InstitutionMembership {
    pub user_id: UserId,
    pub institution_id: InstitutionId,
    pub state: MembershipState,
    pub academic_roles: Vec<AcademicRole>,
    pub permissions: Vec<InstitutionPermission>,
}

impl InstitutionMembership {
    pub fn has_permission(&self, permission: InstitutionPermission) -> bool {
        self.state == MembershipState::Active && self.permissions.contains(&permission)
    }
}
