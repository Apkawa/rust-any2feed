pub mod feed;
mod render;
mod routes;
mod config;

use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;
use std::sync::Arc;
use ::feed::opml::Outline;
use http_server::Route;
use crate::importers::telegram::config::Config;
use crate::importers::telegram::routes::{route_feed, route_opml};
use crate::importers::traits::Importer;

pub struct TelegramImporter {
    pub(crate) config: Arc<Config>,
}


impl Importer for TelegramImporter {
    fn with_config(toml: &str) -> Self {
        TelegramImporter { config: Arc::new(Config::load(toml))}
    }


    fn routes(&self) -> Vec<Route> {
        vec![
            route_feed(&self),
            route_opml(&self),
        ]
    }

    fn opml_outlines(&self) -> Vec<Outline> {
        todo!()
    }
}