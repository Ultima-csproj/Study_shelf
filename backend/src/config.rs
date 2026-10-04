use std::{env, net::SocketAddr, time::Duration};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing or empty DATABASE_URL; see backend/.env.example")]
    MissingDatabaseUrl,
    #[error("invalid environment variable {0}")]
    Environment(&'static str),
    #[error("invalid BIND_ADDRESS")]
    BindAddress(#[source] std::net::AddrParseError),
    #[error("invalid numeric configuration {0}")]
    InvalidNumber(&'static str),
}
pub struct Config {
    pub bind_address: SocketAddr,
    pub database_url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
}
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(error) if error.not_found() => {}
            Err(_) => return Err(ConfigError::Environment(".env")),
        }
        let database_url = env::var("DATABASE_URL").map_err(|_| ConfigError::MissingDatabaseUrl)?;
        if database_url.trim().is_empty() {
            return Err(ConfigError::MissingDatabaseUrl);
        }
        let bind_address = optional("BIND_ADDRESS", "127.0.0.1:3000")?
            .parse()
            .map_err(ConfigError::BindAddress)?;
        let max_connections = number("DATABASE_MAX_CONNECTIONS", "10", 1, 100)?;
        let seconds = number("DATABASE_ACQUIRE_TIMEOUT_SECONDS", "5", 1, 60)?;
        Ok(Self {
            bind_address,
            database_url,
            max_connections,
            acquire_timeout: Duration::from_secs(u64::from(seconds)),
        })
    }
}
fn optional(name: &'static str, default: &str) -> Result<String, ConfigError> {
    match env::var(name) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Ok(default.to_owned()),
        Err(_) => Err(ConfigError::Environment(name)),
    }
}
fn number(name: &'static str, default: &str, min: u32, max: u32) -> Result<u32, ConfigError> {
    let value: u32 = optional(name, default)?
        .parse()
        .map_err(|_| ConfigError::InvalidNumber(name))?;
    if !(min..=max).contains(&value) {
        return Err(ConfigError::InvalidNumber(name));
    }
    Ok(value)
}
