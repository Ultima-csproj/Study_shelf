use axum::Json;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use study_core::error::RepositoryErrorKind;

use crate::catalog::CatalogError;

pub struct ApiError {
    status: StatusCode,
    code: ApiErrorCode,
    message: String,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum ApiErrorCode {
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalError,
    ServiceUnavailable,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: ApiErrorCode,
    message: String,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: ApiErrorCode::BadRequest,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: ApiErrorCode::NotFound,
            message: message.into(),
        }
    }

    pub fn method_not_allowed() -> Self {
        Self {
            status: StatusCode::METHOD_NOT_ALLOWED,
            code: ApiErrorCode::MethodNotAllowed,
            message: "Этот HTTP-метод не поддерживается".to_owned(),
        }
    }

    pub fn internal(error: impl std::error::Error) -> Self {
        tracing::error!(
            error = ?error,
            "Не удалось обработать запрос"
        );

        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: ApiErrorCode::InternalError,
            message: "Не удалось обработать запрос".to_owned(),
        }
    }

    fn unavailable(error: CatalogError) -> Self {
        tracing::warn!(error = ?error, "Хранилище временно недоступно");

        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: ApiErrorCode::ServiceUnavailable,
            message: "Хранилище временно недоступно. Попробуйте позже".to_owned(),
        }
    }
}

impl From<CatalogError> for ApiError {
    fn from(error: CatalogError) -> Self {
        match &error {
            CatalogError::Storage(storage) => match storage.kind() {
                RepositoryErrorKind::Unavailable | RepositoryErrorKind::Retryable => {
                    Self::unavailable(error)
                }
                RepositoryErrorKind::InvalidQuery => {
                    Self::bad_request("Некорректный запрос каталога")
                }
                RepositoryErrorKind::InvalidData | RepositoryErrorKind::Unexpected => {
                    Self::internal(error)
                }
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;

        let body = ErrorResponse {
            error: ErrorDetail {
                code: self.code,
                message: self.message,
            },
        };

        let mut response = (status, Json(body)).into_response();

        if status == StatusCode::METHOD_NOT_ALLOWED {
            response
                .headers_mut()
                .insert(header::ALLOW, HeaderValue::from_static("GET, HEAD"));
        }

        response
    }
}
