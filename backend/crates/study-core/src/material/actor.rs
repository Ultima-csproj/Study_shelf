use crate::identity::User;
use crate::ids::{InstitutionId, MaterialId};
use crate::institution::{InstitutionMembership, InstitutionPermission};

use super::{MaterialCollaborator, MaterialPermission};

/// A snapshot of authenticated identity and access loaded by the application.
/// Rebuild it from current server data for each operation when persistence is added.
pub struct Actor<'a> {
    pub user: &'a User,
    pub memberships: &'a [InstitutionMembership],
    pub collaborators: &'a [MaterialCollaborator],
}

impl Actor<'_> {
    pub(super) fn has_institution_permission(
        &self,
        institution_id: InstitutionId,
        permission: InstitutionPermission,
    ) -> bool {
        self.user.is_active()
            && self.memberships.iter().any(|membership| {
                membership.user_id == self.user.id
                    && membership.institution_id == institution_id
                    && membership.has_permission(permission)
            })
    }

    pub(super) fn has_material_permission(
        &self,
        material_id: MaterialId,
        permission: MaterialPermission,
    ) -> bool {
        self.user.is_active()
            && self.collaborators.iter().any(|access| {
                access.user_id == self.user.id
                    && access.material_id == material_id
                    && access.has_permission(permission)
            })
    }
}
