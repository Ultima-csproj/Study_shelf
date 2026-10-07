use super::{AccountJson, dto::ProfileDto};
use crate::{
    api::{error::ApiError, state::AppState},
    auth::{extractor::Authenticated, password, session},
};
use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use study_core::account::{AccountError, ProfileInput};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Register {
    email: String,
    password: String,
    first_name: String,
    #[serde(default)]
    last_name: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Login {
    email: String,
    password: String,
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Viewer {
    Guest,
    Authenticated { user: Box<ProfileDto> },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Csrf {
    csrf_token: String,
}
pub async fn csrf(State(state): State<AppState>, headers: HeaderMap) -> Result<Response, ApiError> {
    let (session_name, name) = session::names(&state.auth);
    let token = if let Some(secret) = session::cookie(&headers, session_name) {
        if state
            .auth
            .repository
            .session(&session::digest(&secret))
            .await?
            .is_some()
        {
            session::csrf_for_session(&secret)
        } else {
            session::cookie(&headers, name).map_or_else(session::secret, Ok)?
        }
    } else {
        session::cookie(&headers, name).map_or_else(session::secret, Ok)?
    };
    let mut response = Json(Csrf {
        csrf_token: token.clone(),
    })
    .into_response();
    session::set_cookie(
        response.headers_mut(),
        name,
        &token,
        state.auth.session_seconds,
        state.auth.secure_cookie,
    );
    Ok(response)
}
pub async fn viewer(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Viewer>, ApiError> {
    let Some(token) = session::cookie(&headers, session::names(&state.auth).0) else {
        return Ok(Json(Viewer::Guest));
    };
    let digest = session::digest(&token);
    if state.auth.repository.session(&digest).await?.is_none() {
        return Ok(Json(Viewer::Guest));
    }
    match state.auth.repository.profile(&digest).await {
        Ok(profile) => Ok(Json(Viewer::Authenticated {
            user: Box::new(profile.into()),
        })),
        Err(AccountError::SessionExpired) => Ok(Json(Viewer::Guest)),
        Err(error) => Err(error.into()),
    }
}
async fn establish(state: &AppState, user: i64, version: i64) -> Result<Response, ApiError> {
    let token = session::secret()?;
    let csrf = session::csrf_for_session(&token);
    state
        .auth
        .repository
        .create_session(
            user,
            version,
            &session::digest(&token),
            &session::digest(&csrf),
            time::OffsetDateTime::now_utc()
                + time::Duration::seconds(i64::from(state.auth.session_seconds)),
        )
        .await?;
    let profile = state
        .auth
        .repository
        .profile(&session::digest(&token))
        .await?;
    let mut response = Json(Viewer::Authenticated {
        user: Box::new(profile.into()),
    })
    .into_response();
    let (session_name, csrf_name) = session::names(&state.auth);
    session::set_cookie(
        response.headers_mut(),
        session_name,
        &token,
        state.auth.session_seconds,
        state.auth.secure_cookie,
    );
    session::set_cookie(
        response.headers_mut(),
        csrf_name,
        &csrf,
        state.auth.session_seconds,
        state.auth.secure_cookie,
    );
    Ok(response)
}
pub async fn register(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    AccountJson(input): AccountJson<Register>,
) -> Result<Response, ApiError> {
    state.auth.throttle.check(peer.ip())?;
    let email = password::email(&input.email)?;
    let profile = ProfileInput {
        first_name: input.first_name,
        last_name: input.last_name,
        course: None,
        group: String::new(),
        biography: String::new(),
    };
    profile.validate()?;
    let hash = state.auth.passwords.hash(input.password).await?;
    let user = state
        .auth
        .repository
        .register(&email, &hash, &profile)
        .await?;
    establish(&state, user, 1).await
}
pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    AccountJson(input): AccountJson<Login>,
) -> Result<Response, ApiError> {
    state.auth.throttle.check(peer.ip())?;
    let email = password::email(&input.email).map_err(|_| AccountError::InvalidCredentials)?;
    let mut credential = state.auth.repository.credentials(&email).await?;
    let hash = credential
        .as_mut()
        .map(|c| std::mem::take(&mut c.password_hash));
    if !state.auth.passwords.verify(input.password, hash).await? {
        return Err(AccountError::InvalidCredentials.into());
    }
    let credential = credential.ok_or(AccountError::InvalidCredentials)?;
    establish(&state, credential.user_id, credential.password_version).await
}
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let (name, csrf) = session::names(&state.auth);
    if let Some(token) = session::cookie(&headers, name) {
        state
            .auth
            .repository
            .logout(&session::digest(&token))
            .await?;
    }
    let mut response = StatusCode::NO_CONTENT.into_response();
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
pub async fn logout_others(
    State(state): State<AppState>,
    actor: Authenticated,
) -> Result<StatusCode, ApiError> {
    state.auth.repository.logout_others(&actor.digest).await?;
    Ok(StatusCode::NO_CONTENT)
}
