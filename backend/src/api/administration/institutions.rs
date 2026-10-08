use super::AdminQuery;
use crate::{
    api::{account::AccountJson, error::ApiError, state::AppState},
    auth::extractor::Authenticated,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use study_core::{
    account::AccountError,
    institution::{AcademicRole, InstitutionPermission, MembershipState},
};
use study_storage_postgres::administration::{
    AdminOrigin, InstitutionRow, MembershipChange, MembershipDecision, MembershipRow, Page,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InstitutionBody {
    expected_version: i64,
    name: String,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberBody {
    expected_version: i64,
    state: String,
    roles: Vec<String>,
    permissions: Vec<String>,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MemberDecision {
    expected_version: i64,
    role: String,
    accept: bool,
    reason: String,
}
pub async fn institutions(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<AdminQuery>,
) -> Result<Json<Page<InstitutionRow>>, ApiError> {
    Ok(Json(
        state
            .administration
            .institutions(&actor.digest, &p.into())
            .await?,
    ))
}
pub async fn institution(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<InstitutionRow>, ApiError> {
    Ok(Json(
        state.administration.institution(&actor.digest, id).await?,
    ))
}
pub async fn rename(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
    AccountJson(body): AccountJson<InstitutionBody>,
) -> Result<StatusCode, ApiError> {
    state
        .administration
        .rename_institution(
            AdminOrigin::Session(&actor.digest),
            id,
            body.expected_version,
            &body.name,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn members(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
    Query(p): Query<AdminQuery>,
) -> Result<Json<Page<MembershipRow>>, ApiError> {
    Ok(Json(
        state
            .administration
            .members(&actor.digest, id, &p.into())
            .await?,
    ))
}
pub async fn membership(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((institution, user)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<MemberBody>,
) -> Result<StatusCode, ApiError> {
    if body.roles.len() > 2 || body.permissions.len() > 6 {
        return Err(AccountError::Validation("Слишком много ролей или разрешений").into());
    }
    let status = match body.state.as_str() {
        "active" => MembershipState::Active,
        "revoked" => MembershipState::Revoked,
        _ => return Err(AccountError::Validation("Выберите active или revoked").into()),
    };
    let roles = body
        .roles
        .iter()
        .map(|s| role(s))
        .collect::<Result<Vec<_>, _>>()?;
    let permissions = body
        .permissions
        .iter()
        .map(|s| match s.as_str() {
            "manage_members" => Ok(InstitutionPermission::ManageMembers),
            "manage_disciplines" => Ok(InstitutionPermission::ManageDisciplines),
            "create_materials" => Ok(InstitutionPermission::CreateMaterials),
            "manage_materials" => Ok(InstitutionPermission::ManageMaterials),
            "restrict_materials" => Ok(InstitutionPermission::RestrictMaterials),
            "review_restricted_materials" => Ok(InstitutionPermission::ReviewRestrictedMaterials),
            _ => Err(AccountError::Validation(
                "Неизвестное разрешение учреждения",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;
    state
        .administration
        .membership(
            AdminOrigin::Session(&actor.digest),
            user,
            institution,
            Some(body.expected_version),
            MembershipChange {
                state: Some(status),
                roles: Some(roles),
                permissions: Some(permissions),
            },
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn decide_membership(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((institution, user)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<MemberDecision>,
) -> Result<StatusCode, ApiError> {
    state
        .administration
        .decide_membership(
            AdminOrigin::Session(&actor.digest),
            user,
            institution,
            MembershipDecision {
                expected: Some(body.expected_version),
                role: role(&body.role)?,
                accept: body.accept,
                reason: &body.reason,
            },
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
fn role(value: &str) -> Result<AcademicRole, AccountError> {
    match value {
        "student" => Ok(AcademicRole::Student),
        "teacher" => Ok(AcademicRole::Teacher),
        _ => Err(AccountError::Validation("Неизвестный учебный статус")),
    }
}
