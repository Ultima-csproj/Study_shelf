use crate::ids::{MaterialId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialPermission {
    Read,
    EditRevision,
    SubmitForModeration,
    ManageCollaborators,
}

/// Delegated access only; ownership is stored exclusively in MaterialOwner.
#[derive(Debug, Clone)]
pub struct MaterialCollaborator {
    pub material_id: MaterialId,
    pub user_id: UserId,
    pub is_active: bool,
    pub permissions: Vec<MaterialPermission>,
}

impl MaterialCollaborator {
    pub fn has_permission(&self, permission: MaterialPermission) -> bool {
        self.is_active && self.permissions.contains(&permission)
    }
}
