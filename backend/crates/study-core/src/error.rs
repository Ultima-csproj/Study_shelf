use std::error::Error;
use std::fmt;

type ErrorSource = Box<dyn Error + Send + Sync + 'static>;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryErrorKind {
    Unavailable,
    InvalidData,
    Unexpected,
}

#[derive(Debug)]
pub struct RepositoryError {
    kind: RepositoryErrorKind,
    message: String,
    source: Option<ErrorSource>,
}

impl RepositoryError {
    pub fn unavailable(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_source(
            RepositoryErrorKind::Unavailable,
            "storage unavailable",
            source,
        )
    }

    pub fn unexpected(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_source(
            RepositoryErrorKind::Unexpected,
            "unexpected storage error",
            source,
        )
    }

    pub fn invalid_data(message: impl Into<String>) -> Self {
        Self {
            kind: RepositoryErrorKind::InvalidData,
            message: message.into(),
            source: None,
        }
    }

    pub fn invalid_data_with_source(
        message: impl Into<String>,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self::with_source(RepositoryErrorKind::InvalidData, message, source)
    }

    pub fn kind(&self) -> RepositoryErrorKind {
        self.kind
    }

    fn with_source(
        kind: RepositoryErrorKind,
        message: impl Into<String>,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for RepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|source| source as &dyn Error)
    }
}
