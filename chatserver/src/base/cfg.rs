use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Deserialize;

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub client: Addr,
    pub monitor: Addr,
    pub http: Addr,
    pub log: Log,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Addr {
    pub ip: String,
    pub port: u16,
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: String,
    pub filepath: PathBuf,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub uname: String,
    pub password: String,
    pub dbname: String,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub timeout: Option<Duration>,
}

pub fn init_config<P: AsRef<Path>>(cfg_path: P) -> Result<Config> {
    let cfg = config::Config::builder()
        .add_source(config::File::from(cfg_path.as_ref()))
        .build()
        .map_err(|e| Error::ConfigError(e.to_string()))?;
    Ok(cfg.try_deserialize::<Config>().map_err(|e| Error::ConfigError(e.to_string()))?)
}
