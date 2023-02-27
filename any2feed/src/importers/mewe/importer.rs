use std::sync::Arc;
use serde::Deserialize;


use http_server::Route;

use mewe_api::MeweApi;
use crate::importers::mewe::routes::{route_feed, route_media_proxy, route_opml};
use crate::importers::traits::Importer;

pub struct MeweImporter {
    api: Arc<MeweApi>,
}

#[derive(Debug, Deserialize)]
struct Config {
    mewe: MeweConfig
}

#[derive(Debug, Deserialize)]
struct MeweConfig {
    cookies_path: String,
}

impl MeweImporter {
    pub fn api(&self) -> Arc<MeweApi> {
        Arc::clone(&self.api)
    }
}

impl Importer for MeweImporter {
    fn with_config(toml_str: &str) -> Self {
        let config: Config = toml::from_str(toml_str).unwrap();

        let mewe = MeweApi::new(config.mewe.cookies_path.as_str()).unwrap();

        MeweImporter {
            api: Arc::new(mewe)
        }
    }

    fn routes(&self) -> Vec<Route> {
        vec![
            route_opml(self),
            route_feed(self),
            route_media_proxy(self),
        ]
    }
}