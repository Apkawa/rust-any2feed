use std::sync::{Arc, Mutex};
use crate::server::request::HTTPRequest;
use crate::server::error;
use crate::server::response::HTTPResponse;

// pub type ViewCallbackImpl = impl Fn(HTTPRequest) -> error::Result<HTTPResponse>;
pub type ViewCallback = dyn Fn(&HTTPRequest) -> error::Result<HTTPResponse> + Send;

pub struct Route {
    pattern: String,
    callback: Arc<Mutex<ViewCallback>>,
}

impl Route {
    pub fn new(pattern: String,
               callback: impl Fn(&HTTPRequest) -> error::Result<HTTPResponse> + Send + 'static) -> Self {
        Self {
            pattern,
            callback: Arc::new(Mutex::new(callback)),
        }
    }
    pub fn match_path(&self, path: &String) -> bool {
        path == &self.pattern
    }

    pub fn run_cb(&self, request: &HTTPRequest) -> error::Result<HTTPResponse> {
        self.callback.lock().unwrap()(request)
    }
}

#[derive(Default, )]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub routes: Vec<Route>,
}


impl ServerConfig {
    pub fn addr(&self) -> String {
        format!("127.0.0.1:{}", self.port.unwrap_or(12345))
    }
}