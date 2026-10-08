use crate::ids::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserState {
    Active,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPermission {
    ModeratePublicMaterials,
    ManageAccounts,
    ManageInstitutions,
    ManagePlatformPermissions,
    ViewAccountPrivateData,
    RestrictPublicMaterials,
    ReviewRestrictedMaterials,
}
impl PlatformPermission {
    pub fn as_db_value(self) -> &'static str {
        match self {
            Self::ModeratePublicMaterials => "moderate_public_materials",
            Self::ManageAccounts => "manage_accounts",
            Self::ManageInstitutions => "manage_institutions",
            Self::ManagePlatformPermissions => "manage_platform_permissions",
            Self::ViewAccountPrivateData => "view_account_private_data",
            Self::RestrictPublicMaterials => "restrict_public_materials",
            Self::ReviewRestrictedMaterials => "review_restricted_materials",
        }
    }
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
