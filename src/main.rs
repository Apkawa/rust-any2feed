use std::collections::HashMap;
use std::sync::Arc;
use bytes::Bytes;
use rust_any2feed::feed::CDATAElement;
use rust_any2feed::importers::mewe::feed::{get_media_url_from_proxy_path, mewe_feed_to_feed, replace_mewe_media_urls};
use rust_any2feed::importers::mewe::MeweApi;
use rust_any2feed::server;
use rust_any2feed::server::config::{Route, ServerConfig};
use rust_any2feed::server::request::HTTPRequest;
use rust_any2feed::server::http_server;
use rust_any2feed::server::http_server::run;
use rust_any2feed::server::HTTPError::{InvalidRequest, NotFound};
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
    let mewe_3 = Arc::clone(&mewe);

    routes.extend([
        Route::new("/mewe/feed/me/",
                   move |r| {
                       let mewe_feeds = mewe_2.get_my_feeds(None, None).unwrap();
                       let mut feeds = mewe_feed_to_feed(&mewe_feeds).unwrap();
                       feeds.title = CDATAElement("Mewe me feed".to_string());
                       let res = feeds.to_string();
                       let new_url = format!("http://{}/mewe/media", r.config.as_ref().unwrap().addr());
                       let res = replace_mewe_media_urls(
                           res.as_str(), new_url.as_str(),
                       );
                       Ok(HTTPResponse::with_content(res))
                   }),
        Route::new("/mewe/media/*",
                   move |r| {
                       let url = get_media_url_from_proxy_path(&r.path).unwrap();
                       let queries: HashMap<String, String> = url.query_pairs()
                           .into_iter()
                           .map(|(k, v)| (k.to_string(), v.to_string()))
                           .collect();
                       let media_res = mewe_3.get(url.as_str()).unwrap();

                       match media_res.status().as_u16() {
                           200 => {
                               Ok(
                                   HTTPResponse {
                                       status: 200,
                                       content: Some(media_res.bytes().unwrap()),
                                       content_type: queries.get("mime").cloned(),
                                       ..HTTPResponse::default()
                                   }
                               )
                           }
                           404 => Err(NotFound),
                           _ => Err(InvalidRequest),
                       }
                   },
        )
    ]
    );

    let config = ServerConfig {
        routes,
        ..ServerConfig::default()
    };

    run(config).unwrap();
}
