use study_core::error::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoredDataError {
    #[error("unknown value {value:?} for {field}")]
    UnknownValue { field: &'static str, value: String },
    #[error("missing or inconsistent stored relation: {0}")]
    InvalidRelation(&'static str),
}

pub(crate) fn unknown(field: &'static str, value: &str) -> RepositoryError {
    RepositoryError::invalid_data_with_source(
        "invalid stored enum",
        StoredDataError::UnknownValue {
            field,
            value: value.to_owned(),
        },
    )
}
pub(crate) fn invalid_relation(name: &'static str) -> RepositoryError {
    RepositoryError::invalid_data_with_source(
        "invalid stored relation",
        StoredDataError::InvalidRelation(name),
    )
}

pub fn map_sqlx_error(error: sqlx::Error) -> RepositoryError {
    match &error {
        sqlx::Error::PoolClosed
        | sqlx::Error::PoolTimedOut
        | sqlx::Error::Io(_)
        | sqlx::Error::Tls(_) => RepositoryError::unavailable(error),
        sqlx::Error::Decode(_) | sqlx::Error::ColumnDecode { .. } => {
            RepositoryError::invalid_data_with_source("invalid stored representation", error)
        }
        sqlx::Error::Database(db) => {
            let code = db.code();
            match code.as_deref() {
                Some("40001" | "40P01") => RepositoryError::retryable(error),
                Some(value)
                    if value.starts_with("08")
                        || matches!(
                            value,
                            "53300" | "57P01" | "57P02" | "57P03" | "57014" | "55P03"
                        ) =>
                {
                    RepositoryError::unavailable(error)
                }
                Some("XX001" | "XX002") => {
                    RepositoryError::invalid_data_with_source("database corruption", error)
                }
                _ => RepositoryError::unexpected(error),
            }
        }
        _ => RepositoryError::unexpected(error),
    }
}
