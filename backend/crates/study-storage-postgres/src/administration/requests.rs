mod contributors;
mod institutions;
use super::PageRequest;
use serde::Serialize;
use sqlx::FromRow;
use study_core::account::{AccountError, AccountResult};
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RequestRow {
    pub id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub name: String,
    pub description: String,
    pub evidence: Option<String>,
    pub state: String,
    pub version: i64,
    pub created_at: String,
    pub institution_id: Option<i64>,
    pub decision_reason: Option<String>,
}
pub struct InstitutionRequestInput {
    pub name: String,
    pub description: String,
    pub evidence: String,
}
fn validate(p: &PageRequest) -> AccountResult<()> {
    p.validate()?;
    if p.state
        .as_deref()
        .is_some_and(|s| !matches!(s, "pending" | "accepted" | "rejected"))
    {
        return Err(AccountError::Validation("Некорректный статус заявки"));
    }
    Ok(())
}
