use crate::importers::mewe::feed::{mewe_feed_to_feed, replace_mewe_media_urls};
use crate::importers::mewe::importer::MeweImporter;
use feed::opml::{Outline, OPML};
use feed::{CDATAElement, Link, LinkRel};
use http_server::utils::path_params_to_vec;
use http_server::HTTPError::NotFound;
use http_server::{HTTPError, HTTPResponse, Route};
use mewe_api::json::{MeweApiFeedList, MeweApiFeedListNextPageLink, MeweApiHref};
use mewe_api::utils::update_query;
use mewe_api::Url;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn route_opml(importer: &MeweImporter) -> Route {
    let mewe_api = importer.api();
    Route::new("/mewe/feed.opml", move |r| {
        let mut url = r.url();
        url.set_path("/mewe/feed");

        let mut groups = Outline::new("Groups");
        let groups_outlines = mewe_api
            .fetch_groups()
            .unwrap()
            .confirmed_groups
            .iter()
            .map(|g| {
                Outline::new(g.name.as_str()).add_child(
                    g.name.as_str(),
                    Some(format!("{}/group/{}/", url, g.id).as_str()),
                )
            })
            .collect();
        groups.outlines = Some(groups_outlines);

        let mut users = Outline::new("Users");
        let users_outlines = mewe_api
            .get_contacts(true)
            .unwrap()
            .iter()
            .map(|g| {
                Outline::new(g.name.as_str()).add_child(
                    g.name.as_str(),
                    Some(format!("{}/user/{}/", url, g.contact_invite_id).as_str()),
                )
            })
            .collect();
        users.outlines = Some(users_outlines);

        let opml = OPML::new("Mewe feed").add_outline(
            Outline::new("Mewe feeds")
                .add_outline(
                    Outline::new("Home feed")
                        .add_child("Home feed", Some(format!("{url}/me/").as_str())),
                )
                .add_outline(groups)
                .add_outline(users),
        );
        let mut response = HTTPResponse::with_content(opml.to_string());
        response.content_type = Some("text/xml".to_string());
        Ok(response)
    })
}

pub fn route_feed(importer: &MeweImporter) -> Route {
    let mewe_api = importer.api();
    Route::new("/mewe/feed/(me|user|group)/(?:(.+)/|)", move |r| {
        // TODO переработать эту простыню и покрыть тестами
        let page_url = r.query_params.get("page_url");

        let limit = r.query_params.get("limit").and_then(|l| l.parse().ok());
        let pages = r.query_params.get("pages").and_then(|l| l.parse().ok());

        let pairs = path_params_to_vec(r.path_params.as_ref().unwrap());
        let pairs: Vec<Option<&str>> = pairs
            .iter()
            .map(|o| o.as_ref().map(|v| v.as_str()))
            .collect();
        let mut user_id: Option<String> = None;
        let (rel_url, title) = match pairs[1..=2] {
            [Some("me"), ..] => (
                "https://mewe.com/myworld".to_string(),
                "Mewe me feed".to_string(),
            ),
            [Some("user"), Some(invite_id)] => {
                if let Ok(info) = mewe_api.fetch_contact_info(invite_id) {
                    user_id = Some(info.id); // Апи получения информации по id пользователя не нашел
                    (format!("https://mewe.com/i/{invite_id}"), info.name)
                } else {
                    return Err(NotFound);
                }
            }
            [Some("group"), Some(id)] => {
                if let Ok(info) = mewe_api.fetch_group_info(id) {
                    (format!("https://mewe.com/group/{id}"), info.name)
                } else {
                    return Err(NotFound);
                }
            }
            _ => {
                return Err(NotFound);
            }
        };

        let mewe_feeds: Vec<MeweApiFeedList> = if let Some(next_page) = page_url {
            // Паджинация
            mewe_api
                .fetch_feeds(next_page.as_str(), None, None)
                .unwrap()
        } else {
            if pairs[1] != Some("me") {
                // Немного подождем чтоб не мучать мивач
                thread::sleep(Duration::from_millis(100));
            }
            match pairs[1..=2] {
                [Some("me"), ..] => mewe_api.get_my_feeds(limit, pages).unwrap(),
                [Some("user"), Some(_id)] => mewe_api
                    .get_user_feed(user_id.unwrap().as_str(), limit, pages)
                    .unwrap(),
                [Some("group"), Some(id)] => mewe_api.get_group_feed(id, limit, pages).unwrap(),
                _ => {
                    return Err(NotFound);
                }
            }
        };

        let mut feeds = mewe_feed_to_feed(&mewe_feeds).unwrap();

        feeds.title = CDATAElement(title);

        {
            // Next page pagination
            let next_page = mewe_feeds.last().and_then(|f| f.links.as_ref());
            let mut req_url = r.url();
            if let Some(MeweApiFeedListNextPageLink {
                next_page: Some(MeweApiHref { href }),
            }) = next_page
            {
                let href = format!("https://mewe.com{}", href);
                let query = HashMap::from([("page_url", href.as_str())]);
                update_query(&mut req_url, &query);
                feeds
                    .link
                    .push(Link::with_rel(req_url.to_string(), LinkRel::Next))
            }
        }

        let feed_type = {
            let u = Url::parse(rel_url.as_str()).unwrap();
            u.path().to_string()
        };

        feeds.link.push(Link::with_rel(rel_url, LinkRel::Alternate));
        feeds
            .link
            .push(Link::with_rel(r.url().to_string(), LinkRel::_Self));

        for entry in feeds.entries.iter_mut() {
            let mut u = r.url();
            u.query_pairs_mut().clear();
            // Делаем id уникальными в разрезе каждого фида,
            // чтобы иметь возможность иметь дубликаты в разных фидах.
            entry.id = format!("{}/{}", feed_type, entry.id);
        }

        let res = feeds.to_string();
        let new_url = format!("http://{}/mewe/media", r.config.as_ref().unwrap().addr());
        let res = replace_mewe_media_urls(res.as_str(), new_url.as_str());
        let mut response = HTTPResponse::with_content(res);
        response.content_type = Some("text/xml".to_string());
        Ok(response)
    })
}
pub fn route_media_proxy(importer: &MeweImporter) -> Route {
    let mewe_api = importer.api();
    Route::new("/mewe/media/(.*)", move |r| {
        let path = &r.path_params.as_ref().unwrap().get("1").unwrap();
        let path = path.as_ref().unwrap();
        let media_res = mewe_api
            .get(format!("https://mewe.com/{path}").as_str())
            .unwrap();

        match media_res.status().as_u16() {
            200 => {
                let media_headers: HashMap<String, String> = media_res
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                    .collect();
                let content_type = media_headers.get("content-type").cloned();
                Ok(HTTPResponse {
                    status: 200,
                    content: Some(media_res.bytes().unwrap()),
                    content_type,
                    headers: media_headers,
                    ..HTTPResponse::default()
                })
            }
            404 => Err(NotFound),
            _ => Err(HTTPError::InvalidRequest),
        }
    })
}
