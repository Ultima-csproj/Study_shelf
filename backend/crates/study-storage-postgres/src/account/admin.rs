use super::PostgresAccountRepository;
use crate::administration::{AdminOrigin, Administration, MembershipChange, MembershipDecision};
use study_core::{
    account::{AccountError, AccountResult},
    identity::PlatformPermission as P,
    institution::{AcademicRole, InstitutionPermission as I, MembershipState},
};

impl PostgresAccountRepository {
    pub async fn platform_admin(&self, user: i64, grant: bool) -> AccountResult<()> {
        let service = Administration::new(self.pool.clone());
        let permissions = if grant {
            vec![
                P::ModeratePublicMaterials,
                P::ManageAccounts,
                P::ManageInstitutions,
                P::ManagePlatformPermissions,
                P::ViewAccountPrivateData,
                P::RestrictPublicMaterials,
                P::ReviewRestrictedMaterials,
            ]
        } else {
            vec![]
        };
        service
            .platform_permissions(
                AdminOrigin::LocalMaintenance,
                user,
                None,
                &permissions,
                if grant {
                    "CLI: назначение администратора платформы"
                } else {
                    "CLI: отзыв прав администратора платформы"
                },
            )
            .await
    }
    pub async fn institution_admin(
        &self,
        user: i64,
        institution: i64,
        grant: bool,
    ) -> AccountResult<()> {
        let permissions = if grant {
            vec![
                I::ManageMembers,
                I::ManageDisciplines,
                I::CreateMaterials,
                I::ManageMaterials,
                I::RestrictMaterials,
                I::ReviewRestrictedMaterials,
            ]
        } else {
            vec![]
        };
        Administration::new(self.pool.clone())
            .membership(
                AdminOrigin::LocalMaintenance,
                user,
                institution,
                None,
                MembershipChange {
                    state: if grant {
                        Some(MembershipState::Active)
                    } else {
                        None
                    },
                    roles: None,
                    permissions: Some(permissions),
                },
                if grant {
                    "CLI: назначение управляющего"
                } else {
                    "CLI: отзыв прав управляющего"
                },
            )
            .await
    }
    pub async fn decide_membership(
        &self,
        user: i64,
        institution: i64,
        role: &str,
        accept: bool,
    ) -> AccountResult<()> {
        Administration::new(self.pool.clone())
            .decide_membership(
                AdminOrigin::LocalMaintenance,
                user,
                institution,
                MembershipDecision {
                    expected: None,
                    role: academic(role)?,
                    accept,
                    reason: "CLI: решение по заявке на членство",
                },
            )
            .await
    }
    pub async fn link_contributor(
        &self,
        user: i64,
        contributor: i64,
        accept: bool,
    ) -> AccountResult<()> {
        Administration::new(self.pool.clone())
            .decide_contributor(
                AdminOrigin::LocalMaintenance,
                user,
                contributor,
                None,
                accept,
                "CLI: решение по привязке автора",
            )
            .await
    }
}
fn academic(value: &str) -> AccountResult<AcademicRole> {
    match value {
        "student" => Ok(AcademicRole::Student),
        "teacher" => Ok(AcademicRole::Teacher),
        _ => Err(AccountError::Validation("Неизвестный учебный статус")),
    }
}
