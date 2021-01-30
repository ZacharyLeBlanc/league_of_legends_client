use reqwest::Error as ReqwestError;
use std::time::Duration;
use thiserror::Error;
use url::ParseError;

/// Possible error types from this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Unsupported Media Type")]
    UnsupportedMediaType,
    #[error("Too Many Requests")]
    TooManyRequests(Duration),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("Request Error: {0}")]
    RequestError(#[from] ReqwestError),
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),
    #[error("Unknown")]
    Unknown,
}
