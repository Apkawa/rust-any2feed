mod config;
pub mod feed;
mod render;
mod routes;

use crate::feed_sources::error::FeedSourceError;
use crate::feed_sources::telegram::config::Config;
use crate::feed_sources::telegram::routes::{route_feed, route_media_proxy, route_opml};
use crate::feed_sources::traits::FeedSource;
use ::feed::opml::Outline;
use http_server::Route;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct TelegramFeedSource {
    pub(crate) config: Option<Arc<Config>>,
}

impl FeedSource for TelegramFeedSource {
    #[inline]
    fn name(&self) -> String {
        "telegram".to_string()
    }

    fn with_config(&mut self, toml: &str) -> Result<(), FeedSourceError> {
        let config = Config::load(toml);
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
