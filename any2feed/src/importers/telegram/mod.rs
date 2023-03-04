mod config;
pub mod feed;
mod render;
mod routes;

use crate::importers::telegram::config::Config;
use crate::importers::telegram::routes::{route_feed, route_media_proxy, route_opml};
use crate::importers::traits::Importer;
use ::feed::opml::Outline;
use http_server::Route;
use std::sync::Arc;

pub struct TelegramImporter {
    pub(crate) config: Arc<Config>,
}

impl Importer for TelegramImporter {
    fn with_config(toml: &str) -> Self {
        TelegramImporter {
            config: Arc::new(Config::load(toml)),
        }
    }

    fn routes(&self) -> Vec<Route> {
        vec![route_feed(self), route_opml(self), route_media_proxy(self)]
    }

    fn opml_outlines(&self) -> Vec<Outline> {
        todo!()
    }
}
