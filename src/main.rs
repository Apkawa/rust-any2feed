use std::sync::Arc;
use rust_any2feed::importers::mewe::MeweApi;
use rust_any2feed::server;
use rust_any2feed::server::config::{Route, ServerConfig};
use rust_any2feed::server::request::HTTPRequest;
use rust_any2feed::server::http_server;
use rust_any2feed::server::http_server::run;
use rust_any2feed::server::response::HTTPResponse;


fn main_view(request: &HTTPRequest) -> server::Result<HTTPResponse> {
    Ok(HTTPResponse::with_content("OK".to_string()))
}

fn main() {
    let mut routes = vec![
        Route::new("/".to_string(), main_view),
        Route::new("/hello".to_string(),
                   |r|
                       Ok(HTTPResponse::with_content("Hello world".to_string()))),
    ];

    let mewe = MeweApi::new(
        "/home/apkawa/Downloads/mewe.com_cookies.txt".to_string()).unwrap();
    let mewe = Arc::new(mewe);

    routes.push(
        Route::new("/mewe/".to_string(),
               move    |r| {
                   let info = mewe.me_info.as_ref().unwrap();
                   mewe.identify();
                   Ok(HTTPResponse::with_content(info.name.clone()))
               }
        ));

    let config = ServerConfig {
        routes,
        ..ServerConfig::default()
    };

    run(config).unwrap();
}
