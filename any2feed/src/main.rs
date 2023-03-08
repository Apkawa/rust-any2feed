use clap::Parser;

use any2feed::cli::{Commands, CLI};
use any2feed::config::load_config;
use any2feed::feed_sources::FeedSourceList;
use any2feed::logging;

use http_server::{run, HTTPRequest, HTTPResponse, Route, ServerConfig};

fn main_view(_request: &HTTPRequest) -> http_server::Result<HTTPResponse> {
    Ok(HTTPResponse::with_content(
        r#"<html>
        <body>
            <h1>Feeds:</h1>
            <ul>
                <li><a href="/mewe.opml">Mewe OPML</a></li>
                <li><a href="/telegram.opml">Telegram OPML</a></li>
            </ul>
        </body>
    </html>
    "#,
    )
    .set_content_type("text/html"))
}

fn main() {
    let config = load_config();

    let mut routes = vec![Route::new("/", main_view)];

    let feed_source_list = FeedSourceList::get_sources(config.config_text.as_ref().unwrap());
    for feed_source in feed_source_list {
        routes.extend(feed_source.routes());
    }

    let run_args = ServerConfig {
        port: config.server.port,
        threads: config.server.threads,
        routes,
    };

    run(run_args).unwrap();
}
