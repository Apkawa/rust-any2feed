use crate::feed_sources::danbooru::config::{Config, DanbooruFeedSourceConfig};
use crate::feed_sources::danbooru::routes::{route_feed, route_media_proxy, route_opml};
use ::feed::opml::Outline;
use http_server::Route;
use std::sync::Arc;

use crate::feed_sources::error::FeedSourceError;
use crate::feed_sources::traits::FeedSource;

pub mod config;
pub mod data;
pub mod feed;
pub mod routes;

#[derive(Debug, Default)]
pub struct DanbooruFeedSource {
    pub(crate) config: Option<Arc<DanbooruFeedSourceConfig>>,
}

impl FeedSource for DanbooruFeedSource {
    fn name(&self) -> String {
        "danbooru".to_string()
    }

    fn with_config(&mut self, toml: &str) -> Result<(), FeedSourceError> {
        let config: DanbooruFeedSourceConfig = toml::from_str::<Config>(toml)?.danbooru;
        log::debug!("Config: {:?}", config);
        self.config = Some(Arc::new(config));
        Ok(())
    }

    fn routes(&self) -> Vec<Route> {
        vec![route_feed(self), route_opml(self), route_media_proxy(self)]
    }

    fn opml_outlines(&self) -> Vec<Outline> {
        todo!()
    }
}
