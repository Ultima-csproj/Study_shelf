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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MembershipDecision {
    role: AcademicRoleBody,
    accept: bool,
    expected_version: i64,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinkDecision {
    accept: bool,
    expected_version: i64,
    reason: String,
}
pub async fn membership(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((institution, user)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<MembershipDecision>,
) -> Result<StatusCode, ApiError> {
    let role = match body.role {
        AcademicRoleBody::Student => study_core::institution::AcademicRole::Student,
        AcademicRoleBody::Teacher => study_core::institution::AcademicRole::Teacher,
    };
    state
        .administration
        .decide_membership(
            study_storage_postgres::administration::AdminOrigin::Session(&actor.digest),
            user,
            institution,
            study_storage_postgres::administration::MembershipDecision {
                expected: Some(body.expected_version),
                role,
                accept: body.accept,
                reason: &body.reason,
            },
        )
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
        .administration
        .decide_contributor(
            study_storage_postgres::administration::AdminOrigin::Session(&actor.digest),
            user,
            contributor,
            Some(body.expected_version),
            body.accept,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
