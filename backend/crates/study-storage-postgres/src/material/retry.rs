use super::MaterialWriteError;
use std::future::Future;
use std::time::Duration;
use study_core::error::RepositoryErrorKind;

const ATTEMPTS: u32 = 3;

/// Retry only failures that PostgreSQL confirms have aborted the transaction.
/// A network failure during COMMIT has an unknown outcome and is never retried here.
pub(super) async fn run<T, F, Fut>(mut operation: F) -> Result<T, MaterialWriteError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, MaterialWriteError>>,
{
    let mut attempt = 0;
    loop {
        match operation().await {
            Err(MaterialWriteError::Storage(error))
                if error.kind() == RepositoryErrorKind::Retryable && attempt + 1 < ATTEMPTS =>
            {
                tokio::time::sleep(Duration::from_millis(25 * (1_u64 << attempt))).await;
                attempt += 1;
            }
            result => return result,
        }
    }
}
