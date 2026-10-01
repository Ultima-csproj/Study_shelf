use study_core::error::RepositoryError;

pub(crate) fn map_sqlx_error(error: sqlx::Error) -> RepositoryError {
    match &error {
        sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut => RepositoryError::unavailable(error),
        sqlx::Error::ColumnDecode { .. } | sqlx::Error::Decode(_) => {
            RepositoryError::invalid_data_with_source("cannot decode stored value", error)
        }
        sqlx::Error::Database(database) => {
            // SQLite extended result codes retain the primary code in the low byte.
            let primary_code = database
                .code()
                .and_then(|code| code.parse::<i32>().ok())
                .map(|code| code & 0xff);

            match primary_code {
                Some(5 | 6) => RepositoryError::unavailable(error), // BUSY / LOCKED
                Some(11 | 26) => RepositoryError::invalid_data_with_source(
                    "corrupt or invalid SQLite database", // CORRUPT / NOTADB
                    error,
                ),
                _ => RepositoryError::unexpected(error),
            }
        }
        _ => RepositoryError::unexpected(error),
    }
}
