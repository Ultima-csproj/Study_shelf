mod access;
mod audit;
mod guard;
mod institutions;
mod members;
mod overview;
mod requests;
mod users;
pub use access::{AdminAccess, AdminOrigin};
pub use audit::AuditRow;
pub use institutions::InstitutionRow;
pub use members::{MembershipChange, MembershipDecision, MembershipRow};
pub use overview::Overview;
pub use requests::{InstitutionRequestInput, RequestRow};
use serde::Serialize;
use sqlx::PgPool;
use study_core::account::{AccountError, AccountResult};
pub use users::{UserDetail, UserRow};

#[derive(Clone)]
pub struct Administration {
    pool: PgPool,
}
impl Administration {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_count: i64,
}
pub struct PageRequest {
    pub page: u32,
    pub size: u32,
    pub query: String,
    pub state: Option<String>,
}
impl PageRequest {
    pub fn validate(&self) -> AccountResult<()> {
        if self.page == 0 || !(1..=100).contains(&self.size) || self.query.chars().count() > 200 {
            return Err(AccountError::Validation("Некорректная страница или поиск"));
        }
        Ok(())
    }
    pub(crate) fn offset(&self) -> i64 {
        i64::from(self.page - 1) * i64::from(self.size)
    }
    pub(crate) fn finish<T>(&self, items: Vec<T>, total: i64) -> Page<T> {
        Page {
            items,
            total,
            page: self.page,
            page_count: (total + i64::from(self.size) - 1) / i64::from(self.size),
        }
    }
}
