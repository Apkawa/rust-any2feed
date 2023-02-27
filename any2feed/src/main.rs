use std::{env, thread};
use std::collections::HashMap;
use std::env::Args;
use std::fs::read_to_string;
use std::sync::Arc;
use std::time::Duration;

use any2feed::config::MainConfig;
use any2feed::importers::mewe::feed::{mewe_feed_to_feed, replace_mewe_media_urls};
use any2feed::importers::mewe::importer::MeweImporter;
use mewe_api::utils::update_query;
use any2feed::importers::traits::Importer;
use feed::{CDATAElement, Link, LinkRel};
use feed::opml::{OPML, Outline};
use http_server::{HTTPError, HTTPRequest, HTTPResponse, Route, run, ServerConfig};
use http_server::HTTPError::NotFound;
use http_server::utils::path_params_to_vec;
use mewe_api::json::{MeweApiFeedList, MeweApiFeedListNextPageLink, MeweApiHref};
use mewe_api::MeweApi;

fn main_view(_request: &HTTPRequest) -> http_server::Result<HTTPResponse> {
    Ok(HTTPResponse::with_content("OK".to_string()))
}

fn main() {
    // todo cli
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        panic!("Need config path arg");
    }
    let config_path = &args[1];

    let config_str = read_to_string(config_path.as_str()).unwrap();
    let config: MainConfig = toml::from_str(&config_str).unwrap();

    let mut routes = vec![
        Route::new("/", main_view),
        Route::new("/hello",
                   |_r|
                       Ok(HTTPResponse::with_content("Hello world".to_string()))),
    ];

    let mewe_importers = MeweImporter::with_config(&config_str);
    routes.extend(mewe_importers.routes());

    let run_args = ServerConfig {
        port: config.port,
        routes,
        ..ServerConfig::default()
    };

    run(run_args).unwrap();
}
