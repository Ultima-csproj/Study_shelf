use super::session::{cookie, digest, names};
use crate::api::{error::ApiError, state::AppState};
use axum::{extract::FromRequestParts, http::request::Parts};
use study_core::account::AccountError;

pub struct Authenticated {
    pub digest: [u8; 32],
}
impl FromRequestParts<AppState> for Authenticated {
    type Rejection = ApiError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token =
            cookie(&parts.headers, names(&state.auth).0).ok_or(AccountError::Unauthenticated)?;
        let digest = digest(&token);
        state
            .auth
            .repository
            .session(&digest)
            .await?
            .ok_or(AccountError::SessionExpired)?;
        Ok(Self { digest })
    }
}
