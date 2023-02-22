use std::fmt::Error;
use HTTPError::*;
use std::collections::HashMap;
use std::net::TcpStream;
use crate::server::error;
use crate::server::error::HTTPError;

#[derive(Debug, Eq, PartialEq)]
pub enum HTTPMethod {
    HEAD,
    OPTIONS,
    GET,
    POST,
    PUT,
    DELETE,
}

impl Default for HTTPMethod {
    fn default() -> Self {
        HTTPMethod::GET
    }
}

impl HTTPMethod {
    fn from_str(method: &str) -> error::Result<HTTPMethod> {
        match method.to_uppercase().as_str() {
            "OPTIONS" => Ok(HTTPMethod::OPTIONS),
            "HEAD" => Ok(HTTPMethod::HEAD),
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "DELETE" => Ok(HTTPMethod::DELETE),
            _ => Err(InvalidMethod),
        }
    }
}


#[derive(Debug, Default)]
pub struct HTTPRequest<'a > {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub stream: Option<Box<&'a TcpStream>>
}

impl HTTPRequest<'_> {
    pub fn parse(lines: &Vec<String>) -> error::Result<HTTPRequest> {
        let req_head = lines[0]
            .split_whitespace()
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>();

        let mut request = match req_head[..] {
            [method, path, http_version] => {
                HTTPRequest {
                    method: HTTPMethod::from_str(method)?,
                    path: path.to_string(),
                    ..HTTPRequest::default()
                }
            }
            _ => {
                return Err(InvalidRequest);
            }
        };

        for l in &lines[1..] {
            let v = l.split_once(":");
            match v {
                Some((k, v)) => request.headers.insert(k.trim().to_string(), v.trim().to_string()),
                _ => {
                    return Err(InvalidRequest);
                }
            };
        }
        return Ok(request);
    }
}


