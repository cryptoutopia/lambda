use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Token expired")]
    TokenExpired,

    #[error("Token invalid")]
    TokenInvalid,

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Communication with storage error: {0}")]
    DatabaseError(String),

    #[error("Other error: {0}")]
    Other(String),
}