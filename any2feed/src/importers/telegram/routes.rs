use crate::importers::telegram::feed::channel_to_feed;
use crate::importers::telegram::TelegramImporter;
use feed::opml::{Outline, OPML};
use feed::Attribute;
use http_server::HTTPError::NotFound;
use http_server::{HTTPResponse, Route};
use std::sync::Arc;
use telegram::preview_api::TelegramChannelPreviewApi;

pub fn route_feed(importer: &TelegramImporter) -> Route {
    let config = Arc::clone(&importer.config);
    Route::new("/telegram/feed/(.+)/", move |r| {
        // TODO надо что то сделать с этой цепочкой, не очень красиво
        let channel_slug = r
            .path_params
            .as_ref()
            .unwrap()
            .get("1")
            .unwrap()
            .as_ref()
            .unwrap();
        let api = TelegramChannelPreviewApi::new(channel_slug.as_str());
        let channel = api.fetch(config.pages);
        if let Ok(channel) = channel {
            let feed = channel_to_feed(&channel);
            let content = feed.to_string();
            let mut response = HTTPResponse::with_content(content);
            response.content_type = Some("text/xml".to_string());
            Ok(response)
        } else {
            Err(NotFound)
        }
    })
}

pub(crate) fn route_opml(importer: &TelegramImporter) -> Route {
    let config = Arc::clone(&importer.config);
    Route::new("/telegram.opml", move |r| {
        let mut outlines: Vec<Outline> = Vec::with_capacity(config.channels.len());
        let mut url = r.url();
        url.set_path("/telegram/feed");
        for slug in config.channels.keys() {
            outlines
                .push(Outline::new(slug).add_child(slug, Some(format!("{url}/{slug}/").as_str())))
        }
        let opml = OPML::new("Telegram channels").add_outline(Outline {
            title: Attribute("Telegram channels".to_string()),
            outlines: Some(outlines),
            ..Outline::default()
        });
        let content = opml.to_string();
        let mut response = HTTPResponse::with_content(content);
        response.content_type = Some("text/xml".to_string());
        Ok(response)
    })
}
