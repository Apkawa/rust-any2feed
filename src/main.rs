use std::sync::Arc;
use rust_any2feed::importers::mewe::feed::mewe_feed_to_feed;
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
        Route::new("/", main_view),
        Route::new("/hello",
                   |r|
                       Ok(HTTPResponse::with_content("Hello world".to_string()))),
    ];

    let mewe = MeweApi::new(
        "/home/apkawa/Downloads/mewe.com_cookies.txt".to_string()).unwrap();

    let mewe = Arc::new(mewe);
    let mewe_2 = Arc::clone(&mewe);

    routes.extend([
        Route::new("/mewe/feed/me/",
                   move |r| {
                       let mewe_feeds = mewe_2.get_my_feeds(None, None).unwrap();
                       let feeds = mewe_feed_to_feed(&mewe_feeds).unwrap();
                       Ok(HTTPResponse::with_content(feeds.to_string()))
                   }),
    ]
    );

    let config = ServerConfig {
        routes,
        ..ServerConfig::default()
    };

    run(config).unwrap();
}
