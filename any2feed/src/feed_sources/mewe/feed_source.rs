use feed::opml::Outline;
use std::sync::Arc;

use http_server::Route;

use crate::feed_sources::mewe::config::Config;
use crate::feed_sources::mewe::routes::{route_feed, route_media_proxy, route_opml};
use crate::feed_sources::traits::FeedSource;
use mewe_api::MeweApi;

pub struct MeweFeedSource {
    api: Arc<MeweApi>,
}

impl MeweFeedSource {
    pub fn api(&self) -> Arc<MeweApi> {
        Arc::clone(&self.api)
    }
}

impl FeedSource for MeweFeedSource {
    fn with_config(toml_str: &str) -> Self {
        let config: Config = toml::from_str(toml_str).unwrap();

        let mewe = MeweApi::new(config.mewe.cookies_path.as_str()).unwrap();

        MeweFeedSource {
            api: Arc::new(mewe),
        }
    }

    fn routes(&self) -> Vec<Route> {
        vec![route_opml(self), route_feed(self), route_media_proxy(self)]
    }

    fn opml_outlines(&self) -> Vec<Outline> {
        todo!()
    }
}
