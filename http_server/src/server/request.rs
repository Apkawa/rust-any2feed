
use HTTPError::*;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;
use url::Url;
use crate::server::config::ServerConfig;
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
pub struct HTTPRequest<'a> {
    pub method: HTTPMethod,
    pub path: String,
    pub full_path: String,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub stream: Option<Box<&'a TcpStream>>,
    pub config: Option<Arc<ServerConfig>>,
}

impl HTTPRequest<'_> {
    ///
    /// ```
    /// use http_server::HTTPRequest;
    /// let v = vec!["GET / HTTP/1.1".to_string()];
    /// let r = HTTPRequest::parse(&v).unwrap();
    /// assert_eq!(r.path, "/".to_string());
    /// assert_eq!(r.query_params.len(), 0);
    /// let v = vec!["GET /foo?a=1&bar=diez HTTP/1.1".to_string()];
    /// let r = HTTPRequest::parse(&v).unwrap();
    /// assert_eq!(r.path, "/foo".to_string());
    /// assert_eq!(r.query_params.len(), 2);
    /// assert_eq!(r.query_params.get("a").unwrap(), "1");
    /// ```
    pub fn parse(lines: &Vec<String>) -> error::Result<HTTPRequest> {
        let req_head = lines[0]
            .split_whitespace()
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>();

        let mut request = match req_head[..] {
            [method, path, _http_version] => {
                let url = Url::parse(format!("http://example.com{path}").as_str()).unwrap();
                let query_params: HashMap<String, String> = url.query_pairs()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
                HTTPRequest {
                    method: HTTPMethod::from_str(method)?,
                    path: url.path().to_string(),
                    full_path: path.to_string(),
                    query_params,
                    ..HTTPRequest::default()
                }
            }
            _ => {
                return Err(InvalidRequest);
            }
        };

        for l in &lines[1..] {
            let v = l.split_once(':');
            match v {
                Some((k, v)) => request.headers.insert(k.trim().to_string(), v.trim().to_string()),
                _ => {
                    return Err(InvalidRequest);
                }
            };
        }
        Ok(request)
    }

    pub fn url(&self) -> Url {
        let s = format!("http://{}{}", self.config.as_ref().unwrap().addr(), self.full_path);
        Url::parse(s.as_str()).unwrap()
    }
}


