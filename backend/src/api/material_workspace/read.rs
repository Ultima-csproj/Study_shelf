use super::{
    WorkspaceQuery,
    dto::{DecisionDto, PublishedResponse, RevisionResponse},
    id,
};
use crate::{
    api::{error::ApiError, state::AppState},
    auth::extractor::Authenticated,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use study_core::ids::MaterialRevisionId;
use study_storage_postgres::{administration::Page, material::*};

pub async fn restriction_target(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
) -> Result<Json<RestrictionTarget>, ApiError> {
    Ok(Json(
        state
            .materials
            .restriction_target(&actor.digest, id(material)?)
            .await?,
    ))
}

pub async fn restriction_metadata(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
) -> Result<Json<Vec<RestrictionRow>>, ApiError> {
    Ok(Json(
        state
            .materials
            .restriction_metadata(&actor.digest, id(material)?)
            .await?,
    ))
}

pub async fn author_list(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<Page<MaterialHeader>>, ApiError> {
    Ok(Json(
        state
            .materials
            .workspace_list(&actor.digest, MaterialListKind::Author, &q.page())
            .await?,
    ))
}
pub async fn moderation_list(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<Page<MaterialHeader>>, ApiError> {
    Ok(Json(
        state
            .materials
            .workspace_list(&actor.digest, MaterialListKind::Pending, &q.page())
            .await?,
    ))
}
pub async fn restricted_list(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<Page<MaterialHeader>>, ApiError> {
    Ok(Json(
        state
            .materials
            .workspace_list(&actor.digest, MaterialListKind::Restricted, &q.page())
            .await?,
    ))
}
pub async fn workspace(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<Workspace>, ApiError> {
    Ok(Json(
        state
            .materials
            .workspace(&actor.digest, id(material)?, &q.page())
            .await?,
    ))
}
pub async fn placement_entries(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, placement)): Path<(i64, i64)>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<Page<PlacementEntryRow>>, ApiError> {
    Ok(Json(
        state
            .materials
            .placement_entries(&actor.digest, id(material)?, placement, &q.page())
            .await?,
    ))
}
pub async fn revision(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, revision)): Path<(i64, i64)>,
) -> Result<Json<RevisionResponse>, ApiError> {
    let view = state
        .materials
        .authenticated_revision(&actor.digest, id(material)?, MaterialRevisionId(revision))
        .await?;
    let decision = state
        .materials
        .load_moderation(MaterialRevisionId(revision))
        .await?
        .map(DecisionDto::new)
        .transpose()?;
    Ok(Json(RevisionResponse::new(view, decision)?))
}
pub async fn published_public(
    State(state): State<AppState>,
    Path(material): Path<i64>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<PublishedResponse>, ApiError> {
    Ok(Json(PublishedResponse::new(
        state
            .materials
            .published_detail(None, id(material)?, &q.context())
            .await?,
    )?))
}
pub async fn published_authenticated(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<PublishedResponse>, ApiError> {
    Ok(Json(PublishedResponse::new(
        state
            .materials
            .published_detail(Some(&actor.digest), id(material)?, &q.context())
            .await?,
    )?))
}
pub async fn history_public(
    State(state): State<AppState>,
    Path(material): Path<i64>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<PublishedHistory>, ApiError> {
    Ok(Json(
        state
            .materials
            .published_history(None, id(material)?, &q.context(), &q.page())
            .await?,
    ))
}
pub async fn history_authenticated(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<PublishedHistory>, ApiError> {
    Ok(Json(
        state
            .materials
            .published_history(Some(&actor.digest), id(material)?, &q.context(), &q.page())
            .await?,
    ))
}
pub async fn history_revision_public(
    State(state): State<AppState>,
    Path((material, revision)): Path<(i64, i64)>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<RevisionResponse>, ApiError> {
    let view = state
        .materials
        .published_revision(
            None,
            id(material)?,
            MaterialRevisionId(revision),
            &q.context(),
        )
        .await?;
    let decision = state
        .materials
        .load_moderation(MaterialRevisionId(revision))
        .await?
        .map(DecisionDto::new)
        .transpose()?;
    Ok(Json(RevisionResponse::new(view, decision)?))
}
pub async fn history_revision_authenticated(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((material, revision)): Path<(i64, i64)>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<RevisionResponse>, ApiError> {
    let view = state
        .materials
        .published_revision(
            Some(&actor.digest),
            id(material)?,
            MaterialRevisionId(revision),
            &q.context(),
        )
        .await?;
    let decision = state
        .materials
        .load_moderation(MaterialRevisionId(revision))
        .await?
        .map(DecisionDto::new)
        .transpose()?;
    Ok(Json(RevisionResponse::new(view, decision)?))
}
pub async fn collaborators(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<Page<CollaboratorRow>>, ApiError> {
    Ok(Json(
        state
            .materials
            .collaborators(&actor.digest, id(material)?, &q.page())
            .await?,
    ))
}
pub async fn events(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(material): Path<i64>,
    Query(q): Query<WorkspaceQuery>,
) -> Result<Json<Page<MaterialEvent>>, ApiError> {
    Ok(Json(
        state
            .materials
            .material_events(&actor.digest, id(material)?, &q.page())
            .await?,
    ))
}
