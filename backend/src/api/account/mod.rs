pub mod auth;
pub mod collections;
mod dto;
pub mod identity;
pub mod library;
pub mod profile;
use crate::api::error::ApiError;
use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;

pub struct AccountJson<T>(pub T);
impl<S, T> FromRequest<S> for AccountJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiError;
    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(v)| Self(v))
            .map_err(|_: JsonRejection| ApiError::bad_request("Некорректное JSON-тело запроса"))
    }
}
