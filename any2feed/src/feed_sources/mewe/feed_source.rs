use feed::opml::Outline;
use std::sync::Arc;

use http_server::Route;

use crate::feed_sources::error::FeedSourceError;
use crate::feed_sources::mewe::config::Config;
use crate::feed_sources::mewe::routes::{route_feed, route_media_proxy, route_opml};
use crate::feed_sources::traits::FeedSource;
use mewe_api::MeweApi;

#[derive(Debug, Default)]
pub struct MeweFeedSource {
    api: Option<Arc<MeweApi>>,
}

impl MeweFeedSource {
    pub fn api(&self) -> Arc<MeweApi> {
        Arc::clone(self.api.as_ref().unwrap())
    }
}

impl FeedSource for MeweFeedSource {
    #[inline]
    fn name(&self) -> String {
        "mewe".to_string()
    }

    fn with_config(&mut self, toml_str: &str) -> Result<(), FeedSourceError> {
        let config: Config = toml::from_str(toml_str)?;
        log::debug!("Config: {:?}", config);
        let mewe = MeweApi::new(config.mewe.cookies_path.as_str())?;
        self.api = Some(Arc::new(mewe));
        Ok(())
    }

    fn routes(&self) -> Vec<Route> {
        vec![route_opml(self), route_feed(self), route_media_proxy(self)]
    }

    fn opml_outlines(&self) -> Vec<Outline> {
        todo!()
    }
}
