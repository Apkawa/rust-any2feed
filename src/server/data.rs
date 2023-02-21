use std::collections::HashMap;

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


#[derive(Debug, Default)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
