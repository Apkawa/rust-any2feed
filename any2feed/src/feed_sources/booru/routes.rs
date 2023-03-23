use crate::feed_sources::booru::config::BooruSiteConfig;
use crate::feed_sources::booru::feed::{booru_posts_to_feed, Context};
use crate::feed_sources::booru::BooruFeedSource;
use crate::feed_sources::utils::response_from_reqwest_response;
use booru_rs::client::generic::BooruOptionBuilder;
use booru_rs::manager::Engine;
use feed::opml::{Outline, OPML};
use feed::{Attribute, CDATAElement, Element, Person};
use http_server::{HTTPError, HTTPResponse, Route};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;

pub fn route_feed(feed_source: &BooruFeedSource) -> Route {
    let config = Arc::clone(feed_source.config.as_ref().unwrap());
    Route::new("/booru/feed/(.+)/(.+)/", move |r| {
        let key = r
            .path_params
            .as_ref()
            .unwrap()
            .get("1")
            .unwrap()
            .as_ref()
            .unwrap();
        let tag = r.path_params.as_ref().unwrap().get("2").unwrap().to_owned();

        let config = config.sites.get(key).unwrap();

        let mut builder = config.engine.builder().proxy(config.proxy.as_ref());

        if let Some(url) = config.url.as_ref() {
            builder = builder.url(url);
        }
        if let Some(tag) = tag.as_ref() {
            if let Some(tag_config) = config.tags.get(tag) {
                builder = builder.tag(&tag_config.tag).limit(tag_config.limit);
                if let Some(order) = tag_config.order.as_ref() {
                    builder = builder.order(order);
                }
                if let Some(rating) = tag_config.rating.as_ref() {
                    builder = builder.rating(rating);
                }
            } else {
                builder = builder.tag(tag).limit(config.limit);
                if let Some(order) = config.order.as_ref() {
                    builder = builder.order(order);
                }
                if let Some(rating) = config.rating.as_ref() {
                    builder = builder.rating(rating);
                }
            }
        }

        let posts = builder.get();

        if let Ok(posts) = posts {
            let mut proxy_url: Option<Url> = None;
            if config.proxy.is_some() {
                let mut url = r.url();
                url.set_path(format!("/booru/media/{key}/").as_str());
                proxy_url = Some(url);
            }
            let context = Context {
                proxy_url,
                ..Context::default()
            };
            let mut feed = booru_posts_to_feed(posts, Some(&context));
            let base_url = Url::parse(builder.base_url().as_str()).unwrap();
            let host = base_url.host_str().unwrap().to_string();

            feed.title = CDATAElement(host.clone());
            feed.author = Element(Person::new(host, None, None));

            let content = feed.to_string();
            let response =
                HTTPResponse::with_content(content.as_str()).set_content_type("text/xml");
            Ok(response)
        } else {
            Err(HTTPError::InvalidRequest)
        }
    })
}

pub(crate) fn route_opml(feed_source: &BooruFeedSource) -> Route {
    let config = Arc::clone(feed_source.config.as_ref().unwrap());
    Route::new("/booru.opml", move |r| {
        let capacity = config.sites.len();
        let mut outlines: Vec<Outline> = Vec::with_capacity(capacity);
        let mut url = r.url();
        url.set_path("/booru/feed");
        for (key, site) in config.sites.iter() {
            let site_title = site
                .url
                .as_ref()
                .map(|u| {
                    let u = Url::parse(u.as_str()).unwrap();
                    u.host_str().unwrap().to_string()
                })
                .unwrap_or_else(|| site.engine.to_string());

            let mut site_o = Outline::new(site_title.as_str());
            for (tag_key, _) in site.tags.iter() {
                site_o.outlines.push(Outline::new(tag_key).add_child(
                    &site_title,
                    Some(format!("{url}/{key}/{tag_key}/").as_str()),
                ))
            }
            outlines.push(site_o)
        }
        let opml = OPML::new("Booru").add_outline(Outline {
            title: Attribute("Booru".to_string()),
            outlines,
            ..Outline::default()
        });
        let content = opml.to_string();
        let response = HTTPResponse::with_content(content.as_str()).set_content_type("text/xml");
        Ok(response)
    })
}

pub fn route_media_proxy(feed_source: &BooruFeedSource) -> Route {
    let config = Arc::clone(feed_source.config.as_ref().unwrap());
    Route::new(r#"/booru/media/(.+)/"#, move |r| {
        let key = r
            .path_params
            .as_ref()
            .unwrap()
            .get("1")
            .unwrap()
            .as_ref()
            .unwrap();

        let media_url = Url::parse(r.query_params.get("url").unwrap()).map_err(|e| {
            log::error!("{:?}", e);
            HTTPError::InvalidRequest
        })?;

        let site: Option<&BooruSiteConfig> = config.sites.get(key);

        let builder = if let Some(site) = site {
            site.engine.builder().proxy(site.proxy.as_ref())
        } else {
            Engine::Danbooru.builder()
        };

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

        let media_res = builder
            .client()
            .get(media_url.as_str())
            .headers(proxy_headers.clone())
            .send()
            .unwrap();

        match media_res.status().as_u16() {
            200..=299 => Ok(response_from_reqwest_response(media_res)),
            _ => {
                dbg!(&media_url, &media_res);
                Err(HTTPError::InvalidRequest)
            }
        }
    })
}
