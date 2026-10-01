use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct Config {
    pub bind_address: SocketAddr,
    pub database_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let bind_value = match env::var("BIND_ADDRESS") {
            Ok(value) => value,
            Err(env::VarError::NotPresent) => "127.0.0.1:3000".to_owned(),
            Err(error) => return Err(error.into()),
        };

        let bind_address = bind_value
            .parse()
            .context("BIND_ADDRESS должен содержать адрес вида 127.0.0.1:3000")?;

        let mut database_path = match env::var_os("DATABASE_PATH") {
            Some(value) if !value.is_empty() => PathBuf::from(value),
            Some(_) => anyhow::bail!("DATABASE_PATH не должен быть пустым"),
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("data")
                .join("study.sqlite3"),
        };

        if database_path.is_relative() {
            database_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(database_path);
        }

        Ok(Self {
            bind_address,
            database_path,
        })
    }
}
