/// Handmade http server



pub mod data;
pub mod http_server;
pub mod response;
pub mod config;
pub mod error;

pub use error::*;

mod thread_pool;
mod request;
