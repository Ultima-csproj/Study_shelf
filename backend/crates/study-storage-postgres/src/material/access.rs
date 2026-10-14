use std::sync::LazyLock;

#[derive(Clone, Copy)]
enum AccessMode {
    Snapshot,
    Locked,
}
impl AccessMode {
    fn query(self, source: &'static str, locked: &'static LazyLock<String>) -> &'static str {
        match self {
            Self::Snapshot => source,
            Self::Locked => locked,
        }
    }
}
const USER: &str = include_str!("access_sql/user.sql");
const MEMBERSHIP: &str = include_str!("access_sql/membership.sql");
const COLLABORATOR: &str = include_str!("access_sql/collaborator.sql");
static LOCKED_USER: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"{USER}
FOR SHARE OF u"#
    )
});
static LOCKED_MEMBERSHIP: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"{MEMBERSHIP}
FOR SHARE OF m"#
    )
});
static LOCKED_COLLABORATOR: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"{COLLABORATOR}
FOR SHARE OF c"#
    )
});

pub(super) async fn load(
    conn: &mut PgConnection,
    user: UserId,
    material: Option<MaterialId>,
    institution: Option<InstitutionId>,
) -> Result<ActorData, MaterialWriteError> {
    load_in(conn, user, material, institution, AccessMode::Locked).await
}

pub(super) async fn load_snapshot(
    conn: &mut PgConnection,
    user: UserId,
    material: Option<MaterialId>,
    institution: Option<InstitutionId>,
) -> Result<ActorData, MaterialWriteError> {
    load_in(conn, user, material, institution, AccessMode::Snapshot).await
}

use super::write::{Entity, MaterialWriteError};
use crate::error::{map_sqlx_error, unknown};
use sqlx::{FromRow, PgConnection};
use study_core::identity::{PlatformPermission, User, UserState};
use study_core::ids::{InstitutionId, MaterialId, UserId};
use study_core::institution::{
    AcademicRole, InstitutionMembership, InstitutionPermission, MembershipState,
};
use study_core::material::{Actor, MaterialCollaborator, MaterialPermission};

pub(super) struct ActorData {
    user: User,
    memberships: Vec<InstitutionMembership>,
    collaborators: Vec<MaterialCollaborator>,
}
impl ActorData {
    pub fn actor(&self) -> Actor<'_> {
        Actor {
            user: &self.user,
            memberships: &self.memberships,
            collaborators: &self.collaborators,
        }
    }
}
#[derive(FromRow)]
struct UserRow {
    id: i64,
    display_name: String,
    state: String,
    permissions: Vec<String>,
}
#[derive(FromRow)]
struct MembershipRow {
    state: String,
    roles: Vec<String>,
    permissions: Vec<String>,
}
#[derive(FromRow)]
struct CollaboratorRow {
    active: bool,
    permissions: Vec<String>,
}

async fn load_in(
    conn: &mut PgConnection,
    user_id: UserId,
    material_id: Option<MaterialId>,
    institution_id: Option<InstitutionId>,
    mode: AccessMode,
) -> Result<ActorData, MaterialWriteError> {
    let row = sqlx::query_as::<_, UserRow>(mode.query(USER, &LOCKED_USER))
        .bind(user_id.0)
        .fetch_optional(&mut *conn)
        .await
        .map_err(map_sqlx_error)?
        .ok_or(MaterialWriteError::NotFound(Entity::User))?;
    let state = match row.state.as_str() {
        "active" => UserState::Active,
        "disabled" => UserState::Disabled,
        _ => return Err(unknown("user state", &row.state).into()),
    };
    let permissions = row
        .permissions
        .into_iter()
        .map(|value| match value.as_str() {
            "moderate_public_materials" => Ok(PlatformPermission::ModeratePublicMaterials),
            "manage_accounts" => Ok(PlatformPermission::ManageAccounts),
            "manage_institutions" => Ok(PlatformPermission::ManageInstitutions),
            "manage_platform_permissions" => Ok(PlatformPermission::ManagePlatformPermissions),
            "view_account_private_data" => Ok(PlatformPermission::ViewAccountPrivateData),
            "restrict_public_materials" => Ok(PlatformPermission::RestrictPublicMaterials),
            "review_restricted_materials" => Ok(PlatformPermission::ReviewRestrictedMaterials),
            _ => Err(unknown("platform permission", &value)),
        })
        .collect::<Result<_, _>>()?;
    let user = User {
        id: UserId(row.id),
        display_name: row.display_name,
        state,
        permissions,
    };
    let mut memberships = Vec::new();
    if let Some(institution_id) = institution_id {
        let row = sqlx::query_as::<_, MembershipRow>(mode.query(MEMBERSHIP, &LOCKED_MEMBERSHIP))
            .bind(user_id.0)
            .bind(institution_id.0)
            .fetch_optional(&mut *conn)
            .await
            .map_err(map_sqlx_error)?;
        if let Some(row) = row {
            let state = match row.state.as_str() {
                "pending" => MembershipState::Pending,
                "active" => MembershipState::Active,
                "rejected" => MembershipState::Rejected,
                "revoked" => MembershipState::Revoked,
                _ => return Err(unknown("membership state", &row.state).into()),
            };
            let academic_roles = row
                .roles
                .into_iter()
                .map(|value| match value.as_str() {
                    "student" => Ok(AcademicRole::Student),
                    "teacher" => Ok(AcademicRole::Teacher),
                    _ => Err(unknown("academic role", &value)),
                })
                .collect::<Result<_, _>>()?;
            let permissions = row
                .permissions
                .into_iter()
                .map(|value| match value.as_str() {
                    "manage_members" => Ok(InstitutionPermission::ManageMembers),
                    "manage_disciplines" => Ok(InstitutionPermission::ManageDisciplines),
                    "create_materials" => Ok(InstitutionPermission::CreateMaterials),
                    "manage_materials" => Ok(InstitutionPermission::ManageMaterials),
                    "restrict_materials" => Ok(InstitutionPermission::RestrictMaterials),
                    "review_restricted_materials" => {
                        Ok(InstitutionPermission::ReviewRestrictedMaterials)
                    }
                    _ => Err(unknown("institution permission", &value)),
                })
                .collect::<Result<_, _>>()?;
            memberships.push(InstitutionMembership {
                user_id,
                institution_id,
                state,
                academic_roles,
                permissions,
            });
        }
    }
    let mut collaborators = Vec::new();
    if let Some(material_id) = material_id {
        let row =
            sqlx::query_as::<_, CollaboratorRow>(mode.query(COLLABORATOR, &LOCKED_COLLABORATOR))
                .bind(user_id.0)
                .bind(material_id.0)
                .fetch_optional(conn)
                .await
                .map_err(map_sqlx_error)?;
        if let Some(row) = row {
            let permissions = row
                .permissions
                .into_iter()
                .map(|value| match value.as_str() {
                    "read" => Ok(MaterialPermission::Read),
                    "edit_revision" => Ok(MaterialPermission::EditRevision),
                    "submit_for_moderation" => Ok(MaterialPermission::SubmitForModeration),
                    "manage_collaborators" => Ok(MaterialPermission::ManageCollaborators),
                    _ => Err(unknown("material permission", &value)),
                })
                .collect::<Result<_, _>>()?;
            collaborators.push(MaterialCollaborator {
                material_id,
                user_id,
                is_active: row.active,
                permissions,
            });
        }
    }
    Ok(ActorData {
        user,
        memberships,
        collaborators,
    })
}
