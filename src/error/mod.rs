use reqwest::Error as ReqwestError;
use std::time::Duration;
use thiserror::Error;
use url::ParseError;

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
    #[error("ReqwestError")]
    RequestError {
        #[from]
        source: ReqwestError,
    },
    #[error("Parse Error")]
    ParseError {
        #[from]
        source: ParseError,
    },
    #[error("Unknown")]
    Unknown,
}
