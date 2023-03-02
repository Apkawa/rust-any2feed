use std::io;

use reqwest;
use serde_json;

pub type Result<T> = std::result::Result<T, MeweApiError>;

#[derive(Debug)]
pub enum ApiErrorKind {
    IdentifyFail,
    StatusError,
}

#[derive(Debug)]
pub enum MeweApiError {
    ApiError { kind: ApiErrorKind },
    ReqwestError(reqwest::Error),
    IoError(io::Error),
    JsonError(serde_json::Error),
}

impl From<reqwest::Error> for MeweApiError {
    fn from(value: reqwest::Error) -> Self {
        MeweApiError::ReqwestError(value)
    }
}

impl From<io::Error> for MeweApiError {
    fn from(value: io::Error) -> Self {
        MeweApiError::IoError(value)
    }
}

impl From<serde_json::Error> for MeweApiError {
    fn from(value: serde_json::Error) -> Self {
        MeweApiError::JsonError(value)
    }
}
