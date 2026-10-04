use crate::ids::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserState {
    Active,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPermission {
    ModeratePublicMaterials,
}

/// Account data and permissions loaded by the application, never claims from a request.
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub display_name: String,
    pub state: UserState,
    pub permissions: Vec<PlatformPermission>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.id.0 > 0 && self.state == UserState::Active
    }

    pub fn has_permission(&self, permission: PlatformPermission) -> bool {
        self.is_active() && self.permissions.contains(&permission)
    }
}
