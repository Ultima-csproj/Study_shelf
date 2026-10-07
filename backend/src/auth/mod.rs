pub mod extractor;
pub mod password;
pub mod session;
mod throttle;
use std::sync::Arc;
use study_storage_postgres::account::PostgresAccountRepository;
pub use throttle::Throttle;

pub struct AuthService {
    pub repository: PostgresAccountRepository,
    pub passwords: password::Passwords,
    pub throttle: Throttle,
    pub secure_cookie: bool,
    pub session_seconds: u32,
    pub origins: Vec<String>,
}
impl AuthService {
    pub async fn new(
        repository: PostgresAccountRepository,
        secure: bool,
        seconds: u32,
        origins: Vec<String>,
    ) -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(Self {
            repository,
            passwords: password::Passwords::new().await?,
            throttle: Throttle::new(),
            secure_cookie: secure,
            session_seconds: seconds,
            origins,
        }))
    }
}
