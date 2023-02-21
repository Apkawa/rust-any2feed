use rust_any2feed::server;
use rust_any2feed::server::config::{Route, ServerConfig};
use rust_any2feed::server::data::HTTPRequest;
use rust_any2feed::server::http_server;
use rust_any2feed::server::http_server::run;
use rust_any2feed::server::response::HTTPResponse;

fn main_view(request: &HTTPRequest) -> server::Result<HTTPResponse> {
    Ok(HTTPResponse::with_content("OK".to_string()))
}

fn main() {
    let routes = vec![
        Route::new("/".to_string(), main_view),
        Route::new("/hello".to_string(),
                   |r|
                       Ok(HTTPResponse::with_content("Hello world".to_string()))),
    ];
    let config = ServerConfig {
        routes,
        ..ServerConfig::default()
    };
    run(config);
}
