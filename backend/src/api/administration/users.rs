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
use study_core::{account::AccountError, identity::PlatformPermission};
use study_storage_postgres::administration::{AdminOrigin, Page, UserDetail, UserRow};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StateBody {
    expected_version: i64,
    active: bool,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PermissionBody {
    expected_version: i64,
    permissions: Vec<String>,
    reason: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReasonBody {
    reason: String,
}
pub async fn users(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<AdminQuery>,
) -> Result<Json<Page<UserRow>>, ApiError> {
    Ok(Json(
        state.administration.users(&actor.digest, &p.into()).await?,
    ))
}
pub async fn user(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<UserDetail>, ApiError> {
    Ok(Json(state.administration.user(&actor.digest, id).await?))
}
pub async fn user_state(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
    AccountJson(body): AccountJson<StateBody>,
) -> Result<StatusCode, ApiError> {
    state
        .administration
        .set_user_state(
            AdminOrigin::Session(&actor.digest),
            id,
            body.expected_version,
            body.active,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn revoke_sessions(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
    AccountJson(body): AccountJson<ReasonBody>,
) -> Result<StatusCode, ApiError> {
    state
        .administration
        .revoke_sessions(AdminOrigin::Session(&actor.digest), id, &body.reason)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn platform_permissions(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
    AccountJson(body): AccountJson<PermissionBody>,
) -> Result<StatusCode, ApiError> {
    if body.permissions.len() > 7 {
        return Err(AccountError::Validation("Не более пяти глобальных разрешений").into());
    }
    let permissions = body
        .permissions
        .iter()
        .map(|s| match s.as_str() {
            "moderate_public_materials" => Ok(PlatformPermission::ModeratePublicMaterials),
            "manage_accounts" => Ok(PlatformPermission::ManageAccounts),
            "manage_institutions" => Ok(PlatformPermission::ManageInstitutions),
            "manage_platform_permissions" => Ok(PlatformPermission::ManagePlatformPermissions),
            "view_account_private_data" => Ok(PlatformPermission::ViewAccountPrivateData),
            "restrict_public_materials" => Ok(PlatformPermission::RestrictPublicMaterials),
            "review_restricted_materials" => Ok(PlatformPermission::ReviewRestrictedMaterials),
            _ => Err(AccountError::Validation(
                "Неизвестное глобальное разрешение",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;
    state
        .administration
        .platform_permissions(
            AdminOrigin::Session(&actor.digest),
            id,
            Some(body.expected_version),
            &permissions,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
