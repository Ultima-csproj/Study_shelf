use super::{AccountError, AccountResult};
use crate::identity::PlatformPermission;
use crate::ids::{ContributorId, InstitutionId, UserId};
use crate::institution::{AcademicRole, InstitutionPermission, MembershipState};

pub struct ProfileInput {
    pub first_name: String,
    pub last_name: String,
    pub course: Option<i16>,
    pub group: String,
    pub biography: String,
}
impl ProfileInput {
    pub fn validate(&self) -> AccountResult<()> {
        if self.first_name.trim().is_empty()
            || self.first_name.chars().count() > 64
            || self.last_name.chars().count() > 64
            || self.group.chars().count() > 30
            || self.biography.chars().count() > 2000
            || self.course.is_some_and(|value| !(1..=6).contains(&value))
        {
            return Err(AccountError::Validation("Некорректные данные профиля"));
        }
        Ok(())
    }
    pub fn display_name(&self) -> String {
        format!("{} {}", self.first_name.trim(), self.last_name.trim())
            .trim()
            .to_owned()
    }
}
pub struct AccountProfile {
    pub user_id: UserId,
    pub email: String,
    pub email_verified: bool,
    pub display_name: String,
    pub profile: ProfileInput,
    pub primary_contributor_id: ContributorId,
    pub permissions: Vec<PlatformPermission>,
    pub memberships: Vec<MembershipProfile>,
}
pub struct MembershipProfile {
    pub institution_id: InstitutionId,
    pub institution_name: String,
    pub state: MembershipState,
    pub roles: Vec<AcademicRole>,
    pub permissions: Vec<InstitutionPermission>,
}
