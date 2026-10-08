use super::{PostgresAccountRepository, session};
use crate::error::{map_sqlx_error, unknown};
use sqlx::FromRow;
use study_core::account::{
    AccountError, AccountProfile, AccountResult, MembershipProfile, ProfileInput,
};
use study_core::identity::PlatformPermission;
use study_core::ids::{ContributorId, InstitutionId, UserId};
use study_core::institution::{AcademicRole, InstitutionPermission, MembershipState};

#[derive(FromRow)]
struct ProfileRow {
    user_id: i64,
    email: String,
    email_verified: bool,
    display_name: String,
    first_name: String,
    last_name: String,
    course: Option<i16>,
    study_group: String,
    biography: String,
    primary_contributor_id: i64,
    permissions: Vec<String>,
}
#[derive(FromRow)]
struct MembershipRow {
    institution_id: i64,
    name: String,
    state: String,
    roles: Vec<String>,
    permissions: Vec<String>,
}
impl PostgresAccountRepository {
    pub async fn profile(&self, digest: &[u8]) -> AccountResult<AccountProfile> {
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let row = sqlx::query_as::<_, ProfileRow>(
            r#"
                SELECT
                  p.*,
                  c.email,
                  c.email_verified_at IS NOT NULL AS email_verified,
                  u.display_name,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      user_platform_permissions
                    WHERE
                      user_id = u.id
                    ORDER BY
                      permission
                  ) AS permissions
                FROM
                  user_profiles p
                  JOIN users u ON u.id = p.user_id
                  JOIN user_credentials c ON c.user_id = u.id
                WHERE
                  u.id = $1
            "#,
        )
        .bind(user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let memberships = sqlx::query_as::<_, MembershipRow>(
            r#"
                SELECT
                  m.institution_id,
                  i.name,
                  m.state,
                  ARRAY(
                    SELECT
                      role
                    FROM
                      membership_academic_roles
                    WHERE
                      user_id = m.user_id
                      AND institution_id = m.institution_id
                    ORDER BY
                      role
                  ) AS roles,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      membership_permissions
                    WHERE
                      user_id = m.user_id
                      AND institution_id = m.institution_id
                    ORDER BY
                      permission
                  ) AS permissions
                FROM
                  institution_memberships m
                  JOIN institutions i ON i.id = m.institution_id
                WHERE
                  m.user_id = $1
                ORDER BY
                  i.id
            "#,
        )
        .bind(user)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let permissions = row
            .permissions
            .iter()
            .map(|p| match p.as_str() {
                "moderate_public_materials" => Ok(PlatformPermission::ModeratePublicMaterials),
                "manage_accounts" => Ok(PlatformPermission::ManageAccounts),
                "manage_institutions" => Ok(PlatformPermission::ManageInstitutions),
                "manage_platform_permissions" => Ok(PlatformPermission::ManagePlatformPermissions),
                "view_account_private_data" => Ok(PlatformPermission::ViewAccountPrivateData),
                "restrict_public_materials" => Ok(PlatformPermission::RestrictPublicMaterials),
                "review_restricted_materials" => Ok(PlatformPermission::ReviewRestrictedMaterials),
                _ => Err(unknown("platform permission", p)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let memberships = memberships
            .into_iter()
            .map(|m| {
                let state = match m.state.as_str() {
                    "active" => MembershipState::Active,
                    "pending" => MembershipState::Pending,
                    "rejected" => MembershipState::Rejected,
                    "revoked" => MembershipState::Revoked,
                    _ => return Err(unknown("membership state", &m.state)),
                };
                let roles = m
                    .roles
                    .iter()
                    .map(|r| match r.as_str() {
                        "student" => Ok(AcademicRole::Student),
                        "teacher" => Ok(AcademicRole::Teacher),
                        _ => Err(unknown("academic role", r)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let permissions = m
                    .permissions
                    .iter()
                    .map(|p| match p.as_str() {
                        "manage_members" => Ok(InstitutionPermission::ManageMembers),
                        "manage_disciplines" => Ok(InstitutionPermission::ManageDisciplines),
                        "create_materials" => Ok(InstitutionPermission::CreateMaterials),
                        "manage_materials" => Ok(InstitutionPermission::ManageMaterials),
                        "restrict_materials" => Ok(InstitutionPermission::RestrictMaterials),
                        "review_restricted_materials" => {
                            Ok(InstitutionPermission::ReviewRestrictedMaterials)
                        }
                        _ => Err(unknown("membership permission", p)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(MembershipProfile {
                    institution_id: InstitutionId(m.institution_id),
                    institution_name: m.name,
                    state,
                    roles,
                    permissions,
                })
            })
            .collect::<Result<Vec<_>, study_core::error::RepositoryError>>()?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(AccountProfile {
            user_id: UserId(row.user_id),
            email: row.email,
            email_verified: row.email_verified,
            display_name: row.display_name,
            profile: ProfileInput {
                first_name: row.first_name,
                last_name: row.last_name,
                course: row.course,
                group: row.study_group,
                biography: row.biography,
            },
            primary_contributor_id: ContributorId(row.primary_contributor_id),
            permissions,
            memberships,
        })
    }
    pub async fn update_profile(&self, digest: &[u8], input: &ProfileInput) -> AccountResult<()> {
        input.validate()?;
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest).await?.user_id;
        // All identity mutations use the same user lock; session is rechecked inside the transaction.
        sqlx::query(
            r#"
            SELECT
              id
            FROM
              users
            WHERE
              id = $1
            FOR UPDATE
        "#,
        )
        .bind(user)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let name = input.display_name();
        sqlx::query(
            r#"
            UPDATE users
            SET
              display_name = $1
            WHERE
              id = $2
        "#,
        )
        .bind(&name)
        .bind(user)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let result = sqlx::query(
            r#"
            UPDATE user_profiles
            SET
              first_name = $1,
              last_name = $2,
              course = $3,
              study_group = $4,
              biography = $5
            WHERE
              user_id = $6
        "#,
        )
        .bind(input.first_name.trim())
        .bind(input.last_name.trim())
        .bind(input.course)
        .bind(input.group.trim())
        .bind(input.biography.trim())
        .bind(user)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if result.rows_affected() != 1 {
            return Err(AccountError::NotFound);
        }
        sqlx::query(
            r#"
                UPDATE contributors
                SET
                  display_name = $1
                WHERE
                  id = (
                    SELECT
                      primary_contributor_id
                    FROM
                      user_profiles
                    WHERE
                      user_id = $2
                  )
                  AND user_id = $2
            "#,
        )
        .bind(&name)
        .bind(user)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
