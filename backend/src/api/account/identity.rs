use super::AccountJson;
use crate::{
    api::{error::ApiError, state::AppState},
    auth::extractor::Authenticated,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicRoleBody {
    Student,
    Teacher,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MembershipDecision {
    role: AcademicRoleBody,
    accept: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkDecision {
    accept: bool,
}
pub async fn membership(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((institution, user)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<MembershipDecision>,
) -> Result<StatusCode, ApiError> {
    let role = match body.role {
        AcademicRoleBody::Student => "student",
        AcademicRoleBody::Teacher => "teacher",
    };
    state
        .auth
        .repository
        .review_membership(&actor.digest, user, institution, role, body.accept)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn contributor(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((contributor, user)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<LinkDecision>,
) -> Result<StatusCode, ApiError> {
    state
        .auth
        .repository
        .review_contributor(&actor.digest, user, contributor, body.accept)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
