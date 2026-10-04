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

pub(super) async fn load(
    conn: &mut PgConnection,
    user_id: UserId,
    material_id: Option<MaterialId>,
    institution_id: Option<InstitutionId>,
) -> Result<ActorData, MaterialWriteError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
            SELECT
                u.id,
                u.display_name,
                u.state,
                ARRAY(
                    SELECT permission
                    FROM user_platform_permissions
                    WHERE user_id = u.id
                ) AS permissions
            FROM users AS u
            WHERE u.id = $1
            FOR SHARE OF u
        "#,
    )
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
        let row = sqlx::query_as::<_, MembershipRow>(
            r#"
                SELECT
                    m.state,
                    ARRAY(
                        SELECT role
                        FROM membership_academic_roles
                        WHERE user_id = m.user_id
                            AND institution_id = m.institution_id
                    ) AS roles,
                    ARRAY(
                        SELECT permission
                        FROM membership_permissions
                        WHERE user_id = m.user_id
                            AND institution_id = m.institution_id
                    ) AS permissions
                FROM institution_memberships AS m
                WHERE m.user_id = $1
                    AND m.institution_id = $2
                FOR SHARE OF m
            "#,
        )
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
        let row = sqlx::query_as::<_, CollaboratorRow>(
            r#"
                SELECT
                    c.active,
                    ARRAY(
                        SELECT permission
                        FROM collaborator_permissions
                        WHERE user_id = c.user_id
                            AND material_id = c.material_id
                    ) AS permissions
                FROM material_collaborators AS c
                WHERE c.user_id = $1
                    AND c.material_id = $2
                FOR SHARE OF c
            "#,
        )
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
