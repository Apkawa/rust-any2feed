use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

use reqwest::{cookie::Jar, Url};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use serde::__private::de::Borrowed;

use crate::http_client::cookie::{import_cookie_from_file, update_cookie_from_file};
use crate::importers::mewe::json;
use crate::importers::mewe::json::MeweApiFeedListNextPageLink;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

macro_rules! api_mewe {
    () => ( "https://mewe.com/api" )
}
const API_MEWE_IDENTIFY: &str = concat!(api_mewe!(), "/v3/auth/identify");
const API_MEWE_ME_INFO: &str = concat!(api_mewe!(), "/v2/me/info");
const API_MEWE_USER_INFO: &str = concat!(api_mewe!(), "/v2/mycontacts/user/");

const API_MEWE_ALLFEED: &str = concat!(api_mewe!(), "/v2/home/allfeed");
const API_MEWE_USER_FEED: &str = concat!(api_mewe!(), "/v2/home/user/{user_id}/postsfeed");

#[derive(Debug, Default)]
pub struct MeweApi {
    cookies_path: String,
    cookies: Arc<Jar>,
    session: reqwest::blocking::Client,
    headers: Arc<Mutex<HashMap<String, String>>>,
    pub me_info: Option<json::MeweApiSelfProfileInfo>,
}

/// Подсматриваем туда https://github.com/goutsune/mewe-wrapper
impl MeweApi {
    pub fn new(cookies_path: String) -> Option<MeweApi> {
        let jar = import_cookie_from_file(&cookies_path).ok()?;
        let jar = Arc::new(jar);
        let session = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .cookie_provider(Arc::clone(&jar))
            .build().unwrap();
        let mut mewe_api = MeweApi {
            cookies_path,
            cookies: Arc::clone(&jar),
            session,
            ..MeweApi::default()
        };
        mewe_api.identify()?;
        mewe_api.whoami()?;
        Some(mewe_api)
    }

    fn save_cookies(&self, scope_url: &str) -> Option<()> {
        update_cookie_from_file(self.cookies.borrow(),
                                &Url::parse(scope_url).ok()?,
                                &self.cookies_path,
        )
    }

    fn get(&self, url: &str) -> reqwest::Result<Response> {
        let mut rb = self.session.get(url);

        for (k, v) in self.headers.lock().unwrap().iter() {
            rb = rb.header(k, v);
        }
        let result = rb.send()?;
        let cookies: HashMap<String, String> = result.cookies()
            .map(|c| (c.name().to_lowercase(), c.value().to_string()))
            .collect();
        let cookies_len = cookies.len();
        let csrf_token: String = cookies.into_iter()
            .filter(|(n, v) | n == "csrf-token")
            .map(|(_, v)| v)
            .take(1)
            .collect();

        if !csrf_token.is_empty() {
            self.headers.lock().unwrap()
                .insert("x-csrf-token".to_string(), csrf_token);
        }
        if cookies_len > 0 {
            // Если был какой либо set-cookie, сохраняем актуальный стор
            self.save_cookies(&url);
        }
        if (result.status() != 200) {
            dbg!(&result);
            // dbg!(&result.text());
        }
        Ok(result)
    }

    pub fn whoami(&mut self) -> Option<json::MeweApiSelfProfileInfo> {
        let info: json::MeweApiSelfProfileInfo = self.get(API_MEWE_ME_INFO).ok()?.json().unwrap();
        self.me_info = Some(info.clone());
        Some(info)
    }


    pub fn identify(&self) -> Option<()> {
        let result = self.get(API_MEWE_IDENTIFY).ok()?;
        if result.status() == 200 {
            let json = {
                result.json::<json::MeweApiIdentify>().ok()?
            };
            if json.authenticated {
                return Some(());
            }
        }
        None
    }

    pub fn fetch_feed(&self, url: &str, limit: Option<usize>) -> Option<json::MeweApiFeedList> {
        let mut url = Url::parse(url).unwrap();
        let limit = limit.unwrap_or(10);
        url.query_pairs_mut()
            .append_pair("limit", limit.to_string().as_str());
        let response = self.get(url.as_str()).ok()?;
        if response.status() == 200 {
            return Some(response.json::<json::MeweApiFeedList>().unwrap())
        } else {
            return None;
        }
    }

    // Todo iterator
    pub fn fetch_feeds(&self, url: &str, limit: Option<usize>, pages: Option<usize>)
            -> Option<Vec<json::MeweApiFeedList>> {
        self.identify()?;
        let pages = pages.unwrap_or(1);

        let mut result: Vec<json::MeweApiFeedList> = Vec::with_capacity(pages);
        let mut next_page = url.to_string();
        for _ in 0..pages {
            let json = self.fetch_feed(next_page.as_str(), limit)?;
            if let Some(MeweApiFeedListNextPageLink{next_page: Some(page)}) = &json.links {
                next_page = page.href.clone();
            }
            result.push(json)

        }

        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }

    pub fn get_my_feeds(&self, limit: Option<usize>, pages: Option<usize>) -> Option<Vec<json::MeweApiFeedList>> {
        self.fetch_feeds(API_MEWE_ALLFEED, limit, pages)
    }
}


#[cfg(test)]
mod test {
    use std::sync::Arc;

    use reqwest::cookie::Jar;
    use reqwest::Url;
    use crate::importers::mewe::api::MeweApi;


    #[test]
    fn test_identify() {
        let mut mewe = MeweApi::new(
            "/home/apkawa/Downloads/mewe.com_cookies.txt".to_string()).unwrap();
        dbg!(&mewe);
        let info = mewe.whoami().unwrap();
        assert_eq!(info.contact_invite_id, "apkawa");
    }

    #[test]
    fn test_get_feeds() {
        let mut mewe = MeweApi::new(
            "/home/apkawa/Downloads/mewe.com_cookies.txt".to_string()).unwrap();
        let feeds = mewe.get_my_feeds(None, None).unwrap();
        dbg!(&feeds);
    }

    #[test]
    fn example_2() {}
}
