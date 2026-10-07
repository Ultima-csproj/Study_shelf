use super::{AccountJson, dto::ProfileDto};
use crate::{
    api::{error::ApiError, state::AppState},
    auth::{extractor::Authenticated, session},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use study_core::account::{AccountError, ProfileInput};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProfileBody {
    first_name: String,
    last_name: String,
    course: Option<i16>,
    group: String,
    #[serde(default)]
    biography: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordBody {
    current_password: String,
    new_password: String,
}
pub async fn get(
    State(state): State<AppState>,
    actor: Authenticated,
) -> Result<Json<ProfileDto>, ApiError> {
    Ok(Json(
        state.auth.repository.profile(&actor.digest).await?.into(),
    ))
}
pub async fn update(
    State(state): State<AppState>,
    actor: Authenticated,
    AccountJson(p): AccountJson<ProfileBody>,
) -> Result<Json<ProfileDto>, ApiError> {
    state
        .auth
        .repository
        .update_profile(
            &actor.digest,
            &ProfileInput {
                first_name: p.first_name,
                last_name: p.last_name,
                course: p.course,
                group: p.group,
                biography: p.biography,
            },
        )
        .await?;
    get(State(state), actor).await
}
pub async fn password(
    State(state): State<AppState>,
    actor: Authenticated,
    AccountJson(input): AccountJson<PasswordBody>,
) -> Result<Response, ApiError> {
    let credential = state.auth.repository.own_credentials(&actor.digest).await?;
    if !state
        .auth
        .passwords
        .verify(input.current_password, Some(credential.password_hash))
        .await?
    {
        return Err(AccountError::InvalidCredentials.into());
    }
    let hash = state.auth.passwords.hash(input.new_password).await?;
    state
        .auth
        .repository
        .change_password(&actor.digest, credential.password_version, &hash)
        .await?;
    let mut response = StatusCode::NO_CONTENT.into_response();
    let (name, csrf) = session::names(&state.auth);
    session::set_cookie(
        response.headers_mut(),
        name,
        "",
        0,
        state.auth.secure_cookie,
    );
    session::set_cookie(
        response.headers_mut(),
        csrf,
        "",
        0,
        state.auth.secure_cookie,
    );
    Ok(response)
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestDto {
    id: i64,
    name: String,
    state: String,
}
#[derive(Serialize)]
pub struct RequestsDto {
    memberships: Vec<RequestDto>,
    contributors: Vec<RequestDto>,
}
pub async fn requests(
    State(state): State<AppState>,
    actor: Authenticated,
) -> Result<Json<RequestsDto>, ApiError> {
    let result = state
        .auth
        .repository
        .identity_requests(&actor.digest)
        .await?;
    Ok(Json(RequestsDto {
        memberships: result
            .memberships
            .into_iter()
            .map(|r| RequestDto {
                id: r.institution_id,
                name: r.name,
                state: r.state,
            })
            .collect(),
        contributors: result
            .links
            .into_iter()
            .map(|r| RequestDto {
                id: r.contributor_id,
                name: r.display_name,
                state: r.state,
            })
            .collect(),
    }))
}
pub async fn membership(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .auth
        .repository
        .request_membership(&actor.digest, id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn contributor(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .auth
        .repository
        .request_contributor(&actor.digest, id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn material(
    State(state): State<AppState>,
    actor: Authenticated,
    Path(id): Path<i64>,
) -> Result<Response, ApiError> {
    let item = state
        .auth
        .repository
        .material(&actor.digest, id)
        .await?
        .ok_or(AccountError::NotFound)?;
    let dto = crate::api::catalog::dto::material::MaterialDetailDto::try_from(item)
        .map_err(ApiError::internal)?;
    let mut response = Json(dto).into_response();
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("private, no-store"),
    );
    Ok(response)
}

#[derive(Serialize)]
pub struct InstitutionDto {
    id: i64,
    name: String,
}
pub async fn institutions(
    State(state): State<AppState>,
    axum::extract::Query(p): axum::extract::Query<super::library::Pagination>,
) -> Result<Json<super::library::PageDto<InstitutionDto>>, ApiError> {
    let result = state
        .auth
        .repository
        .institutions(p.page, p.page_size)
        .await?;
    let items = result
        .items
        .into_iter()
        .map(|i| InstitutionDto {
            id: i.id,
            name: i.name,
        })
        .collect();
    Ok(Json(super::library::PageDto::new(items, result.total, &p)))
}
