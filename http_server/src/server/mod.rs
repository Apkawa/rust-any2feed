/// Handmade http server



pub mod data;
pub mod http_server;
pub mod response;
pub mod config;
pub mod error;

pub use error::*;

pub mod thread_pool;
pub mod request;
pub mod utils;


pub use request::{HTTPRequest, HTTPMethod};
pub use response::HTTPResponse;
pub use config::{ServerConfig, Route};
pub use self::http_server::run;