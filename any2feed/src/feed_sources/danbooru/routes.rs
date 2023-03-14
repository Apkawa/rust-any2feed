use crate::feed_sources::danbooru::feed::{danbooru_posts_to_feed, Context};
use crate::feed_sources::danbooru::DanbooruFeedSource;
use crate::feed_sources::utils::response_from_reqwest_response;
use booru_rs::client::danbooru::DanbooruClient;
use feed::opml::{Outline, OPML};
use feed::{Attribute, CDATAElement};
use http_server::{HTTPError, HTTPResponse, Route};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;

pub fn route_feed(feed_source: &DanbooruFeedSource) -> Route {
    let config = Arc::clone(feed_source.config.as_ref().unwrap());
    Route::new("/danbooru/feed/(.+)/", move |r| {
        let mut builder = DanbooruClient::builder()
            .limit(config.limit())
            .proxy(config.proxy.as_ref());

        let tag = r.path_params.as_ref().unwrap().get("1").unwrap().to_owned();
        if let Some(tag) = tag.as_ref() {
            builder = builder.tag(tag);
        }

        let posts = builder.build().get();

        if let Ok(posts) = posts {
            let mut proxy_url: Option<Url> = None;
            if config.proxy.is_some() {
                let mut url = r.url();
                url.set_path("/danbooru/media/");
                proxy_url = Some(url);
            }
            let context = Context {
                proxy_url,
                ..Context::default()
            };
            let mut feed = danbooru_posts_to_feed(posts, Some(&context));

            let tag = if let Some(tag) = tag.as_ref() {
                tag.to_owned()
            } else {
                "all".to_string()
            };
            feed.title = CDATAElement(tag);

            let content = feed.to_string();
            let response =
                HTTPResponse::with_content(content.as_str()).set_content_type("text/xml");
            Ok(response)
        } else {
            Err(HTTPError::InvalidRequest)
        }
    })
}

pub(crate) fn route_opml(feed_source: &DanbooruFeedSource) -> Route {
    let config = Arc::clone(feed_source.config.as_ref().unwrap());
    Route::new("/danbooru.opml", move |r| {
        let mut outlines: Vec<Outline> = Vec::with_capacity(config.tags.len());
        let mut url = r.url();
        url.set_path("/danbooru/feed");
        for tag in config.tags.iter() {
            outlines
                .push(Outline::new(&tag).add_child(&tag, Some(format!("{url}/{tag}/").as_str())))
        }
        let opml = OPML::new("Danbooru").add_outline(Outline {
            title: Attribute("Danbooru".to_string()),
            outlines: Some(outlines),
            ..Outline::default()
        });
        let content = opml.to_string();
        let response = HTTPResponse::with_content(content.as_str()).set_content_type("text/xml");
        Ok(response)
    })
}

pub fn route_media_proxy(feed_source: &DanbooruFeedSource) -> Route {
    let config = Arc::clone(feed_source.config.as_ref().unwrap());
    Route::new(r#"/danbooru/media/"#, move |r| {
        let media_url = Url::parse(r.query_params.get("url").unwrap()).unwrap();
        let builder = DanbooruClient::builder()
            .proxy(config.proxy.as_ref())
            .build();

        let mut proxy_headers: HeaderMap = HeaderMap::from_iter(r.headers.iter().map(|(k, v)| {
            (
                HeaderName::from_str(k).unwrap(),
                HeaderValue::from_str(v).unwrap(),
            )
        }));
        proxy_headers.insert(
            "Host",
            HeaderValue::from_str(media_url.host_str().unwrap()).unwrap(),
        );
        proxy_headers.insert(
            "referer",
            HeaderValue::from_str("https://danbooru.donmai.us/posts").unwrap(),
        );

        let media_res = builder
            .client()
            .get(media_url.as_str())
            .headers(proxy_headers.clone())
            .send()
            .unwrap();

        match media_res.status().as_u16() {
            200..=299 => {
                return Ok(response_from_reqwest_response(media_res));
            }
            _ => {
                dbg!(&media_url, &media_res);
                return Err(HTTPError::InvalidRequest);
            }
        }
    })
}
