use std::result;

pub type Result<T> = result::Result<T, HTTPError>;

#[derive(Debug)]
pub enum HTTPError {
    InvalidMethod,
    InvalidRequest,
    NotFound,
}


