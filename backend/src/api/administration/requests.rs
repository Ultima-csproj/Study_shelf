use super::{AdminQuery, DecisionBody};
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
use study_storage_postgres::administration::{
    AdminOrigin, InstitutionRequestInput, Page, RequestRow,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistrationBody {
    name: String,
    description: String,
    evidence: String,
}
pub async fn contributor_requests(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<AdminQuery>,
) -> Result<Json<Page<RequestRow>>, ApiError> {
    Ok(Json(
        state
            .administration
            .contributor_requests(&actor.digest, &p.into())
            .await?,
    ))
}
pub async fn decide_contributor(
    State(state): State<AppState>,
    actor: Authenticated,
    Path((contributor, user)): Path<(i64, i64)>,
    AccountJson(body): AccountJson<DecisionBody>,
) -> Result<StatusCode, ApiError> {
    state
        .administration
        .decide_contributor(
            AdminOrigin::Session(&actor.digest),
            user,
            contributor,
            Some(body.expected_version),
            body.accept,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn institution_requests(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<AdminQuery>,
) -> Result<Json<Page<RequestRow>>, ApiError> {
    Ok(Json(
        state
            .administration
            .institution_requests(&actor.digest, &p.into())
            .await?,
    ))
}
pub async fn decide_institution(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
    AccountJson(body): AccountJson<DecisionBody>,
) -> Result<StatusCode, ApiError> {
    state
        .administration
        .decide_institution(
            AdminOrigin::Session(&actor.digest),
            id,
            body.expected_version,
            body.accept,
            &body.reason,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn own_institution_requests(
    State(state): State<AppState>,
    actor: Authenticated,
    Query(p): Query<AdminQuery>,
) -> Result<Json<Page<RequestRow>>, ApiError> {
    Ok(Json(
        state
            .administration
            .own_institution_requests(&actor.digest, &p.into())
            .await?,
    ))
}
pub async fn request_institution(
    State(state): State<AppState>,
    actor: Authenticated,
    AccountJson(body): AccountJson<RegistrationBody>,
) -> Result<StatusCode, ApiError> {
    state
        .administration
        .request_institution(
            &actor.digest,
            &InstitutionRequestInput {
                name: body.name,
                description: body.description,
                evidence: body.evidence,
            },
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
