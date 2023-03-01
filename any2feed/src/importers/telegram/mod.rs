pub mod feed;
mod render;
mod routes;

use ::feed::opml::Outline;
use http_server::Route;
use crate::importers::telegram::routes::route_feed;
use crate::importers::traits::Importer;

pub struct TelegramImporter {

}


impl Importer for TelegramImporter {
    fn with_config(_toml: &str) -> Self {
        TelegramImporter{}
    }

    fn routes(&self) -> Vec<Route> {
        vec![
            route_feed(&self),
        ]
    }

    fn opml_outlines(&self) -> Vec<Outline> {
        todo!()
    }
}