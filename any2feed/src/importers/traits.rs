use feed::Feed;
use http_server::Route;

pub trait Importer {
    /// Initialize importer with config
    fn with_config(toml: &str) -> Self;
    /// Initialize routes
    fn routes(&self) -> Vec<Route>;

}
