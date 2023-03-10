use crate::feed_sources::error::FeedSourceError;
use feed::opml::Outline;
use http_server::Route;

pub trait FeedSource {
    fn name(&self) -> String;
    /// Initialize with config
    fn with_config(&mut self, toml: &str) -> Result<(), FeedSourceError>;
    /// Initialize routes
    fn routes(&self) -> Vec<Route>;

    fn opml_outlines(&self) -> Vec<Outline>;
}

pub trait RenderContent {
    fn as_dyn(&self) -> &dyn RenderContent
    where
        Self: Sized,
    {
        self
    }
    fn render(&self) -> Option<String>;
}

impl<T: RenderContent> RenderContent for Option<T> {
    fn render(&self) -> Option<String> {
        self.as_ref().and_then(|s| s.render())
    }
}

impl<T: RenderContent> RenderContent for Vec<T> {
    fn render(&self) -> Option<String> {
        Some(self.iter().filter_map(|s| s.render()).collect::<String>())
    }
}
