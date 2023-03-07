use crate::feed_sources::telegram::feed::{channel_to_feed, Context};
use crate::feed_sources::telegram::TelegramFeedSource;
use crate::feed_sources::utils::response_from_reqwest_response;
use feed::opml::{Outline, OPML};
use feed::Attribute;
use http_server::utils::path_params_to_vec;
use http_server::HTTPError::NotFound;
use http_server::{HTTPError, HTTPResponse, Route};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Method;
use std::str::FromStr;
use std::sync::Arc;
use telegram::preview_api::TelegramChannelPreviewApi;

pub fn route_feed(feed_source: &TelegramFeedSource) -> Route {
    let config = Arc::clone(&feed_source.config);
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
            let mut proxy_url = r.url();
            proxy_url.set_path("/telegram/media");

            let context = Context { proxy_url };
            let feed = channel_to_feed(&channel, Some(&context));
            let content = feed.to_string();
            let response =
                HTTPResponse::with_content(content.as_str()).set_content_type("text/xml");
            Ok(response)
        } else {
            Err(NotFound)
        }
    })
}

pub(crate) fn route_opml(feed_source: &TelegramFeedSource) -> Route {
    let config = Arc::clone(&feed_source.config);
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
        let response = HTTPResponse::with_content(content.as_str()).set_content_type("text/xml");
        Ok(response)
    })
}

/*
Для статики делаем урл вида:
/telegram/media/{channel}/{id}/{index}-{field}/?t={timestamp}&url={original_media}
логика следующая, по timestamp если прошло меньше суток - берем оригинальный урл, проксируем.
если 404 и/или timestamp больше суток, тогда
скачиваем пост по ссылке
https://t.me/{channel}/{id}?embed=1&mode=tme&userpic=true
парсим и берем урл у медиа файла и проксируем снова

Потом еще кеш какой нибудь

Рабочий пример:
url=https://cdn4.telegram-cdn.org/file/CPXsqnVMJaZ8SPo6_eLY_Zh_l2vsH9wwEr7sguC5KOLvgGw6eog_MzJXcgR-rltwEsRzGwH5ZFTIveq483rlNGeTkenkV9tLnmNyAkbI5h2ZCpmVTKRDZ87V_88HNm5aaKonpYgcwJW8AfQdIoCC3Nml8g3NJyU6NZi0Qe8Rf7Tw4x41mV74c2EBDBvwLs_k1q5oOMkYNH5rQXc0J4BtxWqaES4tf5R0Y4P5BxLbMYlw-3txRq1Oa4xFLjC_bcXkhM5aDnjm6LPVt0T-5YG_-Ra_WIXkYDdg73cZNGoalkPEAtUqcz-ez9t1ouUFokhBJ8pCsrqtg-bdJtJgE4SpGQ.jpg
*/
pub fn route_media_proxy(_feed_source: &TelegramFeedSource) -> Route {
    // TODO cache
    Route::new(
        r#"/telegram/media/([\w_]+)/(\d+)/(\d+)-(url|thumb_url)/"#,
        move |r| {
            let path_parts: Vec<&str> = path_params_to_vec(r.path_params.as_ref().unwrap())
                .into_iter()
                .map(|v| v.unwrap())
                .collect();
            let [channel_slug, post_id, media_index, field] = path_parts[1..] else { unreachable!() };
            let post_id: usize = post_id.parse().unwrap();
            let media_index: usize = media_index.parse().unwrap();

            let api = TelegramChannelPreviewApi::new(channel_slug);
            let _timestamp: usize = r.query_params.get("t").map(|t| t.parse().unwrap()).unwrap();
            let mut media_url = r.query_params.get("url").unwrap().clone();

            let mut proxy_headers: HeaderMap =
                HeaderMap::from_iter(r.headers.iter().map(|(k, v)| {
                    (
                        HeaderName::from_str(k).unwrap(),
                        HeaderValue::from_str(v).unwrap(),
                    )
                }));
            proxy_headers.insert(
                "referer",
                HeaderValue::from_str(api.embedded_post_url(post_id).as_str()).unwrap(),
            );
            proxy_headers.remove("content-range");

            for i in 0..=1 {
                let media_res = api
                    .request(Method::GET, media_url.as_str())
                    .headers(proxy_headers.clone())
                    .send()
                    .unwrap();

                match media_res.status().as_u16() {
                    200..=299 => {
                        return Ok(response_from_reqwest_response(media_res));
                    }
                    404 => {
                        if i != 0 {
                            break;
                        }
                        // Пробуем получить актуальный урл
                        media_url = api.try_get_new_media_url(post_id, media_index, field);
                    }
                    _ => {
                        dbg!(&media_url, &media_res);
                        return Err(HTTPError::InvalidRequest);
                    }
                }
            }
            Err(NotFound)
        },
    )
}
