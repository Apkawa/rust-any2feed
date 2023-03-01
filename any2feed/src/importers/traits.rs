use feed::opml::Outline;
use http_server::Route;

pub trait Importer {
    /// Initialize importer with config
    fn with_config(toml: &str) -> Self;
    /// Initialize routes
    fn routes(&self) -> Vec<Route>;

    fn opml_outlines(&self) -> Vec<Outline>;
}


pub trait RenderContent {
    fn as_dyn(&self) -> &dyn RenderContent where Self: Sized {
        self
    }
    fn render(&self) -> Option<String>;
}

impl<T: RenderContent> RenderContent for Option<T> {
    fn render(&self) -> Option<String> {
        self.as_ref().map(|s| s.render()).flatten()
    }
}

impl<T: RenderContent> RenderContent for Vec<T> {
    fn render(&self) -> Option<String> {
        Some(
            self.into_iter()
                .filter_map(|s| s.render())
                .collect::<String>()
        )
    }
}