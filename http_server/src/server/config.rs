use crate::server::error;
use crate::server::request::HTTPRequest;
use crate::server::response::HTTPResponse;
use crate::utils::parse_match_captures;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

pub type ViewCallback = dyn Fn(&HTTPRequest) -> error::Result<HTTPResponse> + Send;

pub struct Route {
    pattern: String,
    re: regex::Regex,
    callback: Arc<Mutex<ViewCallback>>,
}

impl Debug for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("pattern", &self.pattern)
            .field("callback", &"Arc<Mutex<ViewCallback>")
            .finish()
    }
}

impl Route {
    pub fn new(
        pattern: &str,
        callback: impl Fn(&HTTPRequest) -> error::Result<HTTPResponse> + Send + 'static,
    ) -> Self {
        Self {
            pattern: pattern.to_string(),
            re: regex::Regex::new(format!(r#"^{pattern}$"#).as_str()).unwrap(),
            callback: Arc::new(Mutex::new(callback)),
        }
    }
    pub fn match_path(&self, path: &str) -> bool {
        self.re.is_match(path)
    }

    pub fn parse_path(&self, path: &str) -> Option<HashMap<String, Option<String>>> {
        parse_match_captures(&self.re, path)
    }

    pub fn run_cb(&self, request: &HTTPRequest) -> error::Result<HTTPResponse> {
        self.callback
            .lock()
            // Ignore poison error
            .unwrap_or_else(|e| e.into_inner())(request)
    }
}

#[derive(Default, Debug)]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub threads: Option<u8>,
    pub routes: Vec<Route>,
}

impl ServerConfig {
    pub fn addr(&self) -> String {
        format!("127.0.0.1:{}", self.port.unwrap_or(12345))
    }
}
