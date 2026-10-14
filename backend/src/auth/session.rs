use super::AuthService;
use crate::api::{error::ApiError, state::AppState};
use axum::{
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, Method, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use rand::{RngCore, rngs::OsRng};
use sha2::{Digest, Sha256};
use study_core::account::AccountError;
use subtle::ConstantTimeEq;

pub fn secret() -> Result<String, ApiError> {
    let mut bytes = [0u8; 32];
    OsRng
        .try_fill_bytes(&mut bytes)
        .map_err(ApiError::internal)?;
    Ok(hex::encode(bytes))
}
pub fn digest(secret: &str) -> [u8; 32] {
    Sha256::digest(secret.as_bytes()).into()
}
pub fn csrf_for_session(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"study-shelf-csrf:");
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}
pub fn cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    for raw in headers.get_all(header::COOKIE) {
        for item in raw.to_str().ok()?.split(';') {
            let Some((key, value)) = item.trim().split_once('=') else {
                continue;
            };
            if key == name && value.len() == 64 && value.bytes().all(|c| c.is_ascii_hexdigit()) {
                return Some(value.to_owned());
            }
        }
    }
    None
}
pub fn names(auth: &AuthService) -> (&'static str, &'static str) {
    if auth.secure_cookie {
        ("__Host-study_session", "__Host-study_csrf")
    } else {
        ("study_session", "study_csrf")
    }
}
pub fn set_cookie(headers: &mut HeaderMap, name: &str, value: &str, seconds: u32, secure: bool) {
    let suffix = if secure { "; Secure" } else { "" };
    let cookie =
        format!("{name}={value}; Path=/; HttpOnly; SameSite=Lax; Max-Age={seconds}{suffix}");
    // Name/value/config are generated internally, never arbitrary client strings.
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).expect("generated cookie is ASCII"),
    );
}
pub async fn guard(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let private = request.uri().path().starts_with("/api/me/")
        || request.uri().path().starts_with("/api/admin/")
        || request.uri().path().starts_with("/api/auth/")
        || request.uri().path() == "/api/viewer";
    if !matches!(
        *request.method(),
        Method::GET | Method::HEAD | Method::OPTIONS
    ) && let Err(error) = csrf(&state.auth, request.headers()).await
    {
        let mut response = error.into_response();
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("private, no-store"),
        );
        return response;
    }
    let mut response = next.run(request).await;
    if private {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("private, no-store"),
        );
    }
    response
}
async fn csrf(auth: &AuthService, headers: &HeaderMap) -> Result<(), ApiError> {
    let origin = headers
        .get(header::ORIGIN)
        .and_then(|h| h.to_str().ok())
        .ok_or(AccountError::Csrf)?;
    if !auth.origins.iter().any(|o| o == origin) {
        return Err(AccountError::Csrf.into());
    }
    let (session_name, csrf_name) = names(auth);
    let expected = cookie(headers, csrf_name).ok_or(AccountError::Csrf)?;
    let actual = headers
        .get("x-csrf-token")
        .and_then(|h| h.to_str().ok())
        .ok_or(AccountError::Csrf)?;
    if !bool::from(expected.as_bytes().ct_eq(actual.as_bytes())) {
        return Err(AccountError::Csrf.into());
    }
    if let Some(token) = cookie(headers, session_name)
        && let Some(session) = auth.repository.session(&digest(&token)).await?
        && !bool::from(session.csrf_hash.ct_eq(&digest(actual)))
    {
        return Err(AccountError::Csrf.into());
    }
    Ok(())
}
