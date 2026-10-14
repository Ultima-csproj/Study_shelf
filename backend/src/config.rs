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
    #[error("invalid authentication configuration: {0}")]
    Authentication(&'static str),
}
pub struct Config {
    pub bind_address: SocketAddr,
    pub database_url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub secure_cookie: bool,
    pub session_seconds: u32,
    pub auth_origins: Vec<String>,
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
        let bind_address: SocketAddr = optional("BIND_ADDRESS", "127.0.0.1:3000")?
            .parse()
            .map_err(ConfigError::BindAddress)?;
        let max_connections = number("DATABASE_MAX_CONNECTIONS", "10", 1, 100)?;
        let seconds = number("DATABASE_ACQUIRE_TIMEOUT_SECONDS", "5", 1, 60)?;
        let secure_cookie = match optional("AUTH_COOKIE_SECURE", "false")?.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                return Err(ConfigError::Authentication(
                    "AUTH_COOKIE_SECURE must be true or false",
                ));
            }
        };
        if !secure_cookie && !bind_address.ip().is_loopback() {
            return Err(ConfigError::Authentication(
                "non-loopback server requires secure cookies and HTTPS proxy",
            ));
        }
        let session_seconds = number("AUTH_SESSION_SECONDS", "604800", 300, 2592000)?;
        let auth_origins:Vec<String>=optional("AUTH_ALLOWED_ORIGINS","http://localhost:5173,http://127.0.0.1:5173,http://localhost:3000,http://127.0.0.1:3000")?
            .split(',').map(|v|v.trim().to_owned()).collect();
        for origin in &auth_origins {
            let uri: axum::http::Uri = origin
                .parse()
                .map_err(|_| ConfigError::Authentication("invalid origin"))?;
            if !matches!(uri.scheme_str(), Some("http" | "https"))
                || uri.authority().is_none()
                || uri.authority().is_some_and(|a| a.as_str().contains('@'))
                || uri.path_and_query().is_some_and(|p| p.as_str() != "/")
                || (secure_cookie && uri.scheme_str() != Some("https"))
            {
                return Err(ConfigError::Authentication(
                    "origins must be exact origins; HTTPS required with secure cookies",
                ));
            }
        }
        Ok(Self {
            bind_address,
            database_url,
            max_connections,
            acquire_timeout: Duration::from_secs(u64::from(seconds)),
            secure_cookie,
            session_seconds,
            auth_origins,
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
