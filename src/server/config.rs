use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use crate::server::request::HTTPRequest;
use crate::server::error;
use crate::server::response::HTTPResponse;

pub type ViewCallback = dyn Fn(&HTTPRequest) -> error::Result<HTTPResponse> + Send;

pub struct Route {
    pattern: String,
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

///
/// ```
/// use rust_any2feed::server::config::match_path;
/// assert!(match_path("/foo/", "/foo/"));
/// assert!(!match_path("/foo/", "/foo/bar"));
/// assert!(match_path("/foo/*", "/foo/bar"));
/// ```
pub fn match_path(pattern: &str, path: &str) -> bool {
    if pattern.ends_with("*") {
        // Примитивный матчинг через *
        path.starts_with(pattern.trim_end_matches("*"))
    } else {
        path == pattern
    }
}

impl Route {
    pub fn new(pattern: &str,
               callback: impl Fn(&HTTPRequest) -> error::Result<HTTPResponse> + Send + 'static) -> Self {
        Self {
            pattern: pattern.to_string(),
            callback: Arc::new(Mutex::new(callback)),
        }
    }
    pub fn match_path(&self, path: &String) -> bool {
        match_path(&self.pattern, path)
    }

    pub fn run_cb(&self, request: &HTTPRequest) -> error::Result<HTTPResponse> {
        self.callback.lock().unwrap()(request)
    }
}

#[derive(Default, Debug)]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub routes: Vec<Route>,
}


impl ServerConfig {
    pub fn addr(&self) -> String {
        format!("127.0.0.1:{}", self.port.unwrap_or(12345))
    }
}