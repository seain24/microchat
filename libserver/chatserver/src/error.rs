use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("parse config file failed, {0}")]
    ConfigError(String),
    #[error("server start failed, {0}")]
    ServerError(String),
    #[error("path is invalid")]
    PathInvalid,
    #[error("connect to database failed, {0}")]
    DatabaseError(String),
}
