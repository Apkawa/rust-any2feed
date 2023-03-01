use http_server::{HTTPResponse, Route};
use http_server::HTTPError::NotFound;
use telegram::preview_api::TelegramChannelPreviewApi;
use crate::importers::telegram::feed::channel_to_feed;
use crate::importers::telegram::TelegramImporter;

pub fn route_feed(_importer: &TelegramImporter) -> Route {
    Route::new(
        "/telegram/feed/(.+)/",
        move |r| {
            let channel_slug = r.path_params.as_ref().unwrap()
                .get("1").unwrap().as_ref().unwrap();
            let api = TelegramChannelPreviewApi::new(channel_slug.as_str());
            let channel = api.fetch();
            if let Ok(channel) = channel {
                let feed = channel_to_feed(&channel);
                let content = feed.to_string();
                let mut response = HTTPResponse::with_content(content);
                response.content_type = Some("text/xml".to_string());
                Ok(response)
            } else {
                Err(NotFound)
            }
        },
    )
}