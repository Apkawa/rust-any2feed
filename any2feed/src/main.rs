use std::collections::HashMap;
use std::sync::Arc;
use feed::{CDATAElement, Link, LinkRel};
use any2feed::importers::mewe::feed::{get_media_url_from_proxy_path, mewe_feed_to_feed, replace_mewe_media_urls};
use any2feed::importers::mewe::json::{MeweApiFeedList, MeweApiFeedListNextPageLink, MeweApiHref};
use any2feed::importers::mewe::MeweApi;
use any2feed::importers::mewe::utils::update_query;
use feed::opml::{OPML, Outline};
use http_server::{run, HTTPError, HTTPRequest, HTTPResponse, Route, ServerConfig};


fn main_view(_request: &HTTPRequest) -> http_server::Result<HTTPResponse> {
    Ok(HTTPResponse::with_content("OK".to_string()))
}

fn main() {
    let mut routes = vec![
        Route::new("/", main_view),
        Route::new("/hello",
                   |_r|
                       Ok(HTTPResponse::with_content("Hello world".to_string()))),
    ];

    let mewe = MeweApi::new(
        "/home/apkawa/Downloads/mewe.com_cookies.txt".to_string()).unwrap();

    let mewe = Arc::new(mewe);
    let mewe_2 = Arc::clone(&mewe);
    let mewe_3 = Arc::clone(&mewe);
    // TODO Уницифировать
    routes.extend([
        Route::new("/mewe/feed.opml",
                   |r| {
                       let mut url = r.url();
                       url.set_path("/mewe/feed");

                       let opml = OPML::new("Mewe feed")
                           .add_outline(
                               Outline::new("Mewe feeds")
                                   .add_child("Home feed", Some(format!("{url}/me/").as_str()))
                                   .add_outline(Outline::new("Groups")
                                       .add_child("TEH", Some(format!("{url}/me/?ten").as_str()))
                                   )
                                   .add_outline(Outline::new("Users")
                                       .add_child("Gotsune", Some(format!("{url}/me/?gotsune").as_str()))
                                   )
                           );
                       let mut response = HTTPResponse::with_content(opml.to_string());
                       response.content_type = Some("text/xml".to_string());
                       Ok(response)
                   },
        ),
        Route::new("/mewe/feed/me/",
                   move |r| {
                       let page_url = r.query_params.get("page_url");
                       let mewe_feeds: Vec<MeweApiFeedList>;
                       if let Some(next_page) = page_url {
                           dbg!(next_page);
                           mewe_feeds = mewe_2.fetch_feeds(next_page.as_str(), None, None).unwrap();
                       } else {
                           let limit = r.query_params.get("limit").and_then(|l| l.parse().ok());
                           let pages = r.query_params.get("pages").and_then(|l| l.parse().ok());
                           mewe_feeds = mewe_2.get_my_feeds(limit, pages).unwrap();
                       }
                       let mut feeds = mewe_feed_to_feed(&mewe_feeds).unwrap();

                       let next_page = mewe_feeds.last()
                           .and_then(|f| f.links.as_ref());
                       feeds.title = CDATAElement("Mewe me feed".to_string());
                       let mut req_url = r.url();
                       if let Some(
                           MeweApiFeedListNextPageLink {
                               next_page: Some(MeweApiHref { href })
                           }
                       ) = next_page {
                           let href = format!("https://mewe.com{}", href);
                           let query = HashMap::from([("page_url", href.as_str())]);
                           update_query(&mut req_url, &query);
                           feeds.link.push(Link::with_rel(req_url.to_string(), LinkRel::Next))
                       }
                       feeds.link.push(Link::with_rel(r.url().to_string(), LinkRel::_Self));
                       let res = feeds.to_string();
                       let new_url = format!("http://{}/mewe/media", r.config.as_ref().unwrap().addr());
                       let res = replace_mewe_media_urls(
                           res.as_str(), new_url.as_str(),
                       );
                       let mut response = HTTPResponse::with_content(res);
                       response.content_type = Some("text/xml".to_string());
                       Ok(response)
                   }),
        Route::new("/mewe/media/*",
                   move |r| {
                       let url = get_media_url_from_proxy_path(&r.path).unwrap();
                       let media_res = mewe_3.get(url.as_str()).unwrap();

                       match media_res.status().as_u16() {
                           200 => {
                               let media_headers: HashMap<String, String> = media_res.headers().iter()
                                   .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                                   .collect();
                               let content_type = media_headers.get("content-type").cloned();
                               Ok(
                                   HTTPResponse {
                                       status: 200,
                                       content: Some(media_res.bytes().unwrap()),
                                       content_type,
                                       headers: media_headers,
                                       ..HTTPResponse::default()
                                   }
                               )
                           }
                           404 => Err(HTTPError::NotFound),
                           _ => Err(HTTPError::InvalidRequest),
                       }
                   },
        )
    ]
    );

    let config = ServerConfig {
        routes,
        ..ServerConfig::default()
    };

    run(config).unwrap();
}
