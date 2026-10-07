use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use rand::RngCore;
use std::sync::Arc;
use study_core::account::AccountError;
use thiserror::Error;
use tokio::sync::Semaphore;
use zeroize::Zeroizing;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("{0}")]
    Account(#[from] AccountError),
    #[error("password worker failed")]
    Task(#[source] tokio::task::JoinError),
    #[error("stored password hash is invalid")]
    InvalidHash,
    #[error("password hashing failed")]
    Hashing,
    #[error("password verification failed")]
    Verification,
    #[error("secure random source failed")]
    Entropy,
}

pub struct Passwords {
    workers: Arc<Semaphore>,
    dummy: String,
}
impl Passwords {
    pub async fn new() -> anyhow::Result<Self> {
        let dummy =
            tokio::task::spawn_blocking(|| make_hash("unusable-random-dummy-password")).await??;
        Ok(Self {
            workers: Arc::new(Semaphore::new(2)),
            dummy,
        })
    }
    pub async fn hash(&self, password: String) -> Result<String, PasswordError> {
        validate(&password)?;
        let permit = self
            .workers
            .clone()
            .try_acquire_owned()
            .map_err(|_| AccountError::RateLimited)?;
        let password = Zeroizing::new(password);
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            make_hash(&password)
        })
        .await
        .map_err(PasswordError::Task)?
    }
    pub async fn verify(
        &self,
        password: String,
        hash: Option<String>,
    ) -> Result<bool, PasswordError> {
        if password.len() > 1024 {
            return Err(AccountError::InvalidCredentials.into());
        }
        let permit = self
            .workers
            .clone()
            .try_acquire_owned()
            .map_err(|_| AccountError::RateLimited)?;
        let known = hash.is_some();
        let hash = hash.unwrap_or_else(|| self.dummy.clone());
        let password = Zeroizing::new(password);
        let valid = tokio::task::spawn_blocking(move || -> Result<bool, PasswordError> {
            let _permit = permit;
            let parsed = PasswordHash::new(&hash).map_err(|_| PasswordError::InvalidHash)?;
            match Argon2::default().verify_password(password.as_bytes(), &parsed) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::Password) => Ok(false),
                Err(_) => Err(PasswordError::Verification),
            }
        })
        .await
        .map_err(PasswordError::Task)??;
        Ok(known && valid)
    }
}
fn make_hash(password: &str) -> Result<String, PasswordError> {
    let mut bytes = [0u8; 16];
    OsRng
        .try_fill_bytes(&mut bytes)
        .map_err(|_| PasswordError::Entropy)?;
    let salt = SaltString::encode_b64(&bytes).map_err(|_| PasswordError::Hashing)?;
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| PasswordError::Hashing)
}
pub fn validate(password: &str) -> Result<(), AccountError> {
    if password.chars().count() < 12 || password.len() > 1024 {
        return Err(AccountError::Validation(
            "Пароль: минимум 12 символов, максимум 1024 байта",
        ));
    }
    Ok(())
}
pub fn email(value: &str) -> Result<String, AccountError> {
    let value = value.trim().to_ascii_lowercase();
    let mut parts = value.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();
    if value.len() > 254
        || local.is_empty()
        || local.len() > 64
        || domain.is_empty()
        || !domain.contains('.')
        || parts.next().is_some()
        || !value.is_ascii()
        || value
            .bytes()
            .any(|c| c.is_ascii_whitespace() || c.is_ascii_control())
    {
        return Err(AccountError::Validation("Укажите корректный email"));
    }
    Ok(value)
}
