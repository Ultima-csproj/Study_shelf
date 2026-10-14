use super::{dto::ContentBody, id};
use crate::{
    api::{account::AccountJson, error::ApiError, state::AppState},
    auth::extractor::Authenticated,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use study_core::{
    ids::{InstitutionId, MaterialRevisionId, UserId},
    material::{MaterialLifecycle, MaterialPermission, PublicationState},
};
use study_storage_postgres::material::{MaterialStateChange, NewMaterialInput, RevisionTransition};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateBody {
    owner_institution_id: Option<i64>,
    content: ContentBody,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveBody {
    expected_version: i64,
    content: ContentBody,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ForkBody {
    expected_version: i64,
    source_revision_id: Option<i64>,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevisionAction {
    Discard,
    Submit,
    Withdraw,
    Approve,
    Reject,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionBody {
    expected_version: i64,
    action: RevisionAction,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateAction {
    Archive,
    Restore,
    Unpublish,
    Republish,
    Private,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StateBody {
    expected_version: i64,
    action: StateAction,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictBody {
    expected_version: i64,
    institution_id: Option<i64>,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VersionBody {
    expected_version: i64,
    reason: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionBody {
    Read,
    EditRevision,
    SubmitForModeration,
    ManageCollaborators,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollaboratorBody {
    expected_version: i64,
    active: bool,
    permissions: Vec<PermissionBody>,
    reason: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Created {
    material_id: i64,
    revision_id: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionCreated {
    revision_id: i64,
}
#[derive(Serialize)]
pub struct Saved {
    version: i64,
}

pub async fn create(
    State(state): State<AppState>,
    actor: Authenticated,
    AccountJson(body): AccountJson<CreateBody>,
) -> Result<Json<Created>, ApiError> {
    if body.owner_institution_id.is_some_and(|id| id <= 0) {
        return Err(ApiError::bad_request("Некорректное учреждение"));
    }
    let (material_id, revision_id) = state
        .materials
        .create_authenticated(
            &actor.digest,
            NewMaterialInput {
                institution: body.owner_institution_id.map(InstitutionId),
                content: body.content.input()?,
            },
            "Первоначальное создание материала",
        )
        .await?;
    Ok(Json(Created {
        material_id,
        revision_id,
    }))
}
pub async fn fork(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    AccountJson(body): AccountJson<ForkBody>,
) -> Result<Json<RevisionCreated>, ApiError> {
    let revision_id = state
        .materials
        .fork_authenticated(
            &actor.digest,
            id(material)?,
            body.expected_version,
            body.source_revision_id.map(MaterialRevisionId),
            &body.reason,
        )
        .await?;
    Ok(Json(RevisionCreated { revision_id }))
}
pub async fn save(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, revision)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<SaveBody>,
) -> Result<Json<Saved>, ApiError> {
    let version = state
        .materials
        .save_authenticated(
            &actor.digest,
            id(material)?,
            MaterialRevisionId(revision),
            body.expected_version,
            body.content.input()?,
            &body.reason,
        )
        .await?;
    Ok(Json(Saved { version }))
}
pub async fn transition(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, revision)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<TransitionBody>,
) -> Result<StatusCode, ApiError> {
    let action = match body.action {
        RevisionAction::Discard => RevisionTransition::Discard,
        RevisionAction::Submit => RevisionTransition::Submit,
        RevisionAction::Withdraw => RevisionTransition::Withdraw,
        RevisionAction::Approve => RevisionTransition::Approve,
        RevisionAction::Reject => RevisionTransition::Reject,
    };
    state
        .materials
        .transition_authenticated(
            &actor.digest,
            id(material)?,
            MaterialRevisionId(revision),
            body.expected_version,
            action,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn material_state(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    AccountJson(body): AccountJson<StateBody>,
) -> Result<StatusCode, ApiError> {
    let action = match body.action {
        StateAction::Archive => MaterialStateChange::Lifecycle(MaterialLifecycle::Archived),
        StateAction::Restore => MaterialStateChange::Lifecycle(MaterialLifecycle::Active),
        StateAction::Unpublish => MaterialStateChange::Publication(PublicationState::Unpublished),
        StateAction::Republish => MaterialStateChange::Publication(PublicationState::Published),
        StateAction::Private => MaterialStateChange::Private,
    };
    state
        .materials
        .change_state_authenticated(
            &actor.digest,
            id(material)?,
            body.expected_version,
            action,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn restrict(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    AccountJson(body): AccountJson<RestrictBody>,
) -> Result<StatusCode, ApiError> {
    state
        .materials
        .restrict_authenticated(
            &actor.digest,
            id(material)?,
            body.expected_version,
            body.institution_id,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn unrestrict(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, restriction)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<VersionBody>,
) -> Result<StatusCode, ApiError> {
    state
        .materials
        .unrestrict_authenticated(
            &actor.digest,
            id(material)?,
            body.expected_version,
            restriction,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn set_collaborator(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, user)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<CollaboratorBody>,
) -> Result<StatusCode, ApiError> {
    if body.permissions.len() > 4 || user <= 0 {
        return Err(ApiError::bad_request(
            "Некорректный участник или разрешения",
        ));
    }
    let permissions: Vec<MaterialPermission> = body
        .permissions
        .into_iter()
        .map(|permission| match permission {
            PermissionBody::Read => MaterialPermission::Read,
            PermissionBody::EditRevision => MaterialPermission::EditRevision,
            PermissionBody::SubmitForModeration => MaterialPermission::SubmitForModeration,
            PermissionBody::ManageCollaborators => MaterialPermission::ManageCollaborators,
        })
        .collect();
    state
        .materials
        .set_collaborator_authenticated(
            &actor.digest,
            id(material)?,
            body.expected_version,
            study_storage_postgres::material::CollaboratorChange {
                user: UserId(user),
                active: body.active,
                permissions,
            },
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn detach_placement(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, placement)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<VersionBody>,
) -> Result<StatusCode, ApiError> {
    state
        .materials
        .detach_placement_authenticated(
            &actor.digest,
            id(material)?,
            body.expected_version,
            placement,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn detach_entry(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, entry)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<VersionBody>,
) -> Result<StatusCode, ApiError> {
    state
        .materials
        .detach_entry_authenticated(
            &actor.digest,
            id(material)?,
            body.expected_version,
            entry,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
