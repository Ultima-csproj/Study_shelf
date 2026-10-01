use axum::Json;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use study_core::error::RepositoryErrorKind;

use crate::catalog::CatalogError;

pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "bad_request",
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "not_found",
            message: message.into(),
        }
    }

    pub fn method_not_allowed() -> Self {
        Self {
            status: StatusCode::METHOD_NOT_ALLOWED,
            code: "method_not_allowed",
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
            code: "internal_error",
            message: "Не удалось обработать запрос".to_owned(),
        }
    }

    fn unavailable(error: CatalogError) -> Self {
        tracing::warn!(error = ?error, "Хранилище временно недоступно");

        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "service_unavailable",
            message: "Хранилище временно недоступно. Попробуйте позже".to_owned(),
        }
    }
}

impl From<CatalogError> for ApiError {
    fn from(error: CatalogError) -> Self {
        match &error {
            CatalogError::Storage(storage) => match storage.kind() {
                RepositoryErrorKind::Unavailable => Self::unavailable(error),
                RepositoryErrorKind::InvalidData | RepositoryErrorKind::Unexpected => {
                    Self::internal(error)
                }
            },
            CatalogError::MissingDiscipline { .. } | CatalogError::MissingTeacher { .. } => {
                Self::internal(error)
            }
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
