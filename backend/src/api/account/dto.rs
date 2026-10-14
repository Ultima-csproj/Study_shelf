use serde::Serialize;
use study_core::account::AccountProfile;
use study_core::identity::PlatformPermission;
use study_core::institution::{AcademicRole, InstitutionPermission, MembershipState};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDto {
    pub id: i64,
    pub name: String,
    pub initials: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: bool,
    pub role: &'static str,
    pub course: Option<i16>,
    pub group: String,
    pub biography: String,
    pub primary_contributor_id: i64,
    pub permissions: Vec<&'static str>,
    pub memberships: Vec<MembershipDto>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipDto {
    institution_id: i64,
    institution_name: String,
    state: &'static str,
    roles: Vec<&'static str>,
    permissions: Vec<&'static str>,
}
impl From<AccountProfile> for ProfileDto {
    fn from(p: AccountProfile) -> Self {
        let permissions = p
            .permissions
            .iter()
            .map(|v| match v {
                PlatformPermission::ModeratePublicMaterials => "moderate_public_materials",
                PlatformPermission::ManageAccounts => "manage_accounts",
                PlatformPermission::ManageInstitutions => "manage_institutions",
                PlatformPermission::ManagePlatformPermissions => "manage_platform_permissions",
                PlatformPermission::ViewAccountPrivateData => "view_account_private_data",
                PlatformPermission::RestrictPublicMaterials => "restrict_public_materials",
                PlatformPermission::ReviewRestrictedMaterials => "review_restricted_materials",
            })
            .collect();
        let mut role = "user";
        for m in &p.memberships {
            if m.state == MembershipState::Active {
                if m.roles.contains(&AcademicRole::Student) && role == "user" {
                    role = "student";
                }
                if m.roles.contains(&AcademicRole::Teacher) {
                    role = "teacher";
                }
            }
        }
        if p.permissions
            .contains(&PlatformPermission::ManagePlatformPermissions)
        {
            role = "admin";
        }
        let initials = format!(
            "{}{}",
            p.profile.first_name.chars().next().unwrap_or(' '),
            p.profile.last_name.chars().next().unwrap_or(' ')
        )
        .trim()
        .to_uppercase();
        Self {
            id: p.user_id.0,
            name: p.display_name,
            initials,
            first_name: p.profile.first_name,
            last_name: p.profile.last_name,
            email: p.email,
            email_verified: p.email_verified,
            role,
            course: p.profile.course,
            group: p.profile.group,
            biography: p.profile.biography,
            primary_contributor_id: p.primary_contributor_id.0,
            permissions,
            memberships: p
                .memberships
                .into_iter()
                .map(|m| MembershipDto {
                    institution_id: m.institution_id.0,
                    institution_name: m.institution_name,
                    state: match m.state {
                        MembershipState::Active => "active",
                        MembershipState::Pending => "pending",
                        MembershipState::Rejected => "rejected",
                        MembershipState::Revoked => "revoked",
                    },
                    roles: m
                        .roles
                        .iter()
                        .map(|r| match r {
                            AcademicRole::Student => "student",
                            AcademicRole::Teacher => "teacher",
                        })
                        .collect(),
                    permissions: m
                        .permissions
                        .iter()
                        .map(|r| match r {
                            InstitutionPermission::ManageMembers => "manage_members",
                            InstitutionPermission::ManageDisciplines => "manage_disciplines",
                            InstitutionPermission::CreateMaterials => "create_materials",
                            InstitutionPermission::ManageMaterials => "manage_materials",
                            InstitutionPermission::RestrictMaterials => "restrict_materials",
                            InstitutionPermission::ReviewRestrictedMaterials => {
                                "review_restricted_materials"
                            }
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
