use sea_orm::DbErr;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("user {0} is not exist")]
    UserNotExist(String),
    #[error("username is dumplicate")]
    UsernameDumplicate,
    #[error("user phone is dumplicate")]
    PhoneDumplicate,
    #[error("email is dumplicate")]
    EmailDumplicate,
    #[error("database error, {0}")]
    DabaseError(DbErr),
}
