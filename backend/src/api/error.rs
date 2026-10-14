use axum::Json;
use axum::http::StatusCode;
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
    InvalidCredentials,
    Unauthenticated,
    SessionExpired,
    Forbidden,
    Csrf,
    EmailTaken,
    VersionConflict,
    ContributorAlreadyLinked,
    RateLimited,
    LastPlatformAdministrator,
    LastInstitutionManager,
    AlreadyDecided,
    AccountDisabled,
    PendingInstitutionRequest,
    MembershipAlreadyPending,
    MembershipAlreadyActive,
    ContributorEvidenceMissing,
    MaterialVersionConflict,
    WorkingRevisionExists,
    RestrictionExists,
    ScopeMismatch,
    InvalidReason,
    PendingReviewActive,
    RevisionConflict,
    InvalidRevisionState,
    MaterialArchived,
    NoPublishedRevision,
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

    fn unavailable(error: impl std::error::Error) -> Self {
        tracing::warn!(error = ?error, "Хранилище временно недоступно");

        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: ApiErrorCode::ServiceUnavailable,
            message: "Хранилище временно недоступно. Попробуйте позже".to_owned(),
        }
    }
}

impl From<study_core::account::AccountError> for ApiError {
    fn from(error: study_core::account::AccountError) -> Self {
        use study_core::account::AccountError as E;
        let (status, code) = match &error {
            E::Storage(storage) => {
                return match storage.kind() {
                    RepositoryErrorKind::Unavailable | RepositoryErrorKind::Retryable => {
                        Self::unavailable(error)
                    }
                    RepositoryErrorKind::InvalidQuery => Self::bad_request("Некорректный запрос"),
                    RepositoryErrorKind::InvalidData | RepositoryErrorKind::Unexpected => {
                        Self::internal(error)
                    }
                };
            }
            E::Validation(_) => (StatusCode::BAD_REQUEST, ApiErrorCode::BadRequest),
            E::InvalidCredentials => (StatusCode::UNAUTHORIZED, ApiErrorCode::InvalidCredentials),
            E::Unauthenticated => (StatusCode::UNAUTHORIZED, ApiErrorCode::Unauthenticated),
            E::SessionExpired => (StatusCode::UNAUTHORIZED, ApiErrorCode::SessionExpired),
            E::Forbidden => (StatusCode::FORBIDDEN, ApiErrorCode::Forbidden),
            E::Csrf => (StatusCode::FORBIDDEN, ApiErrorCode::Csrf),
            E::EmailTaken => (StatusCode::CONFLICT, ApiErrorCode::EmailTaken),
            E::NotFound => (StatusCode::NOT_FOUND, ApiErrorCode::NotFound),
            E::VersionConflict => (StatusCode::CONFLICT, ApiErrorCode::VersionConflict),
            E::ContributorAlreadyLinked => {
                (StatusCode::CONFLICT, ApiErrorCode::ContributorAlreadyLinked)
            }
            E::RateLimited => (StatusCode::TOO_MANY_REQUESTS, ApiErrorCode::RateLimited),
            E::LastPlatformAdministrator => (
                StatusCode::CONFLICT,
                ApiErrorCode::LastPlatformAdministrator,
            ),
            E::LastInstitutionManager => {
                (StatusCode::CONFLICT, ApiErrorCode::LastInstitutionManager)
            }
            E::AlreadyDecided => (StatusCode::CONFLICT, ApiErrorCode::AlreadyDecided),
            E::AccountDisabled => (StatusCode::CONFLICT, ApiErrorCode::AccountDisabled),
            E::PendingInstitutionRequest => (
                StatusCode::CONFLICT,
                ApiErrorCode::PendingInstitutionRequest,
            ),
            E::MembershipAlreadyPending => {
                (StatusCode::CONFLICT, ApiErrorCode::MembershipAlreadyPending)
            }
            E::MembershipAlreadyActive => {
                (StatusCode::CONFLICT, ApiErrorCode::MembershipAlreadyActive)
            }
            E::ContributorEvidenceMissing => (
                StatusCode::CONFLICT,
                ApiErrorCode::ContributorEvidenceMissing,
            ),
        };
        Self {
            status,
            code,
            message: error.to_string(),
        }
    }
}
impl From<crate::auth::password::PasswordError> for ApiError {
    fn from(error: crate::auth::password::PasswordError) -> Self {
        match error {
            crate::auth::password::PasswordError::Account(error) => error.into(),
            other => Self::internal(other),
        }
    }
}

impl From<CatalogError> for ApiError {
    fn from(error: CatalogError) -> Self {
        match error {
            CatalogError::Storage(storage) => Self::from(storage),
        }
    }
}

impl From<study_core::error::RepositoryError> for ApiError {
    fn from(error: study_core::error::RepositoryError) -> Self {
        match error.kind() {
            RepositoryErrorKind::Unavailable | RepositoryErrorKind::Retryable => {
                Self::unavailable(error)
            }
            RepositoryErrorKind::InvalidQuery => Self::bad_request("Некорректный запрос"),
            RepositoryErrorKind::InvalidData | RepositoryErrorKind::Unexpected => {
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

        (status, Json(body)).into_response()
    }
}

impl From<study_storage_postgres::material::MaterialWriteError> for ApiError {
    fn from(error: study_storage_postgres::material::MaterialWriteError) -> Self {
        use study_core::material::MaterialError as D;
        use study_storage_postgres::material::MaterialWriteError as E;
        let (status, code, message) = match error {
            E::Account(error) => return error.into(),
            E::Storage(error) => return error.into(),
            E::Timestamp(error) => return Self::internal(error),
            E::InvalidFixture => return Self::bad_request("Некорректные fixture данные"),
            E::NotFound(_) => {
                return Self::not_found("Материал или редакция не найдены либо недоступны");
            }
            E::DraftConflict { .. } => (
                StatusCode::CONFLICT,
                ApiErrorCode::VersionConflict,
                "Черновик изменился. Обновите данные".to_owned(),
            ),
            E::MaterialVersionConflict => (
                StatusCode::CONFLICT,
                ApiErrorCode::MaterialVersionConflict,
                "Материал изменился. Обновите данные".to_owned(),
            ),
            E::WorkingRevisionExists => (
                StatusCode::CONFLICT,
                ApiErrorCode::WorkingRevisionExists,
                "У материала уже есть рабочая редакция".to_owned(),
            ),
            E::RestrictionExists => (
                StatusCode::CONFLICT,
                ApiErrorCode::RestrictionExists,
                "Ограничение в этой области уже действует".to_owned(),
            ),
            E::ScopeMismatch => (
                StatusCode::BAD_REQUEST,
                ApiErrorCode::ScopeMismatch,
                "Материал не относится к указанной области".to_owned(),
            ),
            E::InvalidReason => (
                StatusCode::BAD_REQUEST,
                ApiErrorCode::InvalidReason,
                "Причина: от 3 до 2000 символов".to_owned(),
            ),
            E::PendingReviewActive => (
                StatusCode::CONFLICT,
                ApiErrorCode::PendingReviewActive,
                "Сначала отзовите редакцию из очереди модерации".to_owned(),
            ),
            E::RevisionConflict { .. } => (
                StatusCode::CONFLICT,
                ApiErrorCode::RevisionConflict,
                "Опубликованная база изменилась. Пересмотрите черновик на новой базе".to_owned(),
            ),
            E::Domain(D::PermissionDenied) => (
                StatusCode::FORBIDDEN,
                ApiErrorCode::Forbidden,
                "Недостаточно прав для этого действия".to_owned(),
            ),
            E::Domain(D::InvalidRevisionState { .. }) => (
                StatusCode::CONFLICT,
                ApiErrorCode::InvalidRevisionState,
                "Состояние редакции изменилось либо действие недопустимо".to_owned(),
            ),
            E::Domain(D::MaterialArchived) => (
                StatusCode::CONFLICT,
                ApiErrorCode::MaterialArchived,
                "Сначала восстановите материал из архива".to_owned(),
            ),
            E::Domain(D::NoPublishedRevision) => (
                StatusCode::CONFLICT,
                ApiErrorCode::NoPublishedRevision,
                "У материала ещё нет опубликованной редакции".to_owned(),
            ),
            E::Domain(D::MaterialIsPrivate) => (
                StatusCode::CONFLICT,
                ApiErrorCode::InvalidRevisionState,
                "Для публичного доступа отправьте новую редакцию на модерацию".to_owned(),
            ),
            E::Domain(error) => (
                StatusCode::BAD_REQUEST,
                ApiErrorCode::BadRequest,
                error.to_string(),
            ),
        };
        Self {
            status,
            code,
            message,
        }
    }
}
