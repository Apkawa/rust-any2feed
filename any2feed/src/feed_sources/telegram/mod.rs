mod config;
pub mod feed;
mod render;
mod routes;

use crate::feed_sources::telegram::config::Config;
use crate::feed_sources::telegram::routes::{route_feed, route_media_proxy, route_opml};
use crate::feed_sources::traits::FeedSource;
use ::feed::opml::Outline;
use http_server::Route;
use std::sync::Arc;

pub struct TelegramFeedSource {
    pub(crate) config: Arc<Config>,
}

impl FeedSource for TelegramFeedSource {
    fn with_config(toml: &str) -> Self {
        let config = Config::load(toml);
        log::debug!("Config: {:?}", config);
        TelegramFeedSource {
            config: Arc::new(config),
        }
    }

    fn routes(&self) -> Vec<Route> {
        vec![route_feed(self), route_opml(self), route_media_proxy(self)]
    }

    fn opml_outlines(&self) -> Vec<Outline> {
        todo!()
    }
}
