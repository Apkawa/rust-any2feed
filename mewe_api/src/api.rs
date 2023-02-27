use std::borrow::{Borrow};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use reqwest::{cookie::Jar, Url};
use reqwest::blocking::Response;

use reqwest_mozilla_cookie::{import_cookie_from_file, update_cookie_from_file};
use crate::json;
use crate::json::MeweApiFeedListNextPageLink;
use crate::utils::update_query;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

macro_rules! api_mewe {
    () => ( "https://mewe.com/api" )
}
const API_MEWE_IDENTIFY: &str = concat!(api_mewe!(), "/v3/auth/identify");
const API_MEWE_ME_INFO: &str = concat!(api_mewe!(), "/v2/me/info");
// const API_MEWE_USER_INFO: &str = concat!(api_mewe!(), "/v2/mycontacts/user/");

pub const API_MEWE_ALLFEED: &str = concat!(api_mewe!(), "/v2/home/allfeed");
pub const API_MEWE_USER_FEED: &str = concat!(api_mewe!(), "/v2/home/user/{user_id}/postsfeed");
pub const API_MEWE_GROUP_FEED: &str = concat!(api_mewe!(), "/v3/group/{group_id}/postsfeed");

const API_MEWE_CONTACT_INFO: &str = concat!(api_mewe!(), "/v2/mycontacts/user?inviteId={invite_id}");
const API_MEWE_CONTACTS_ALL: &str = concat!(api_mewe!(), "/v2/mycontacts/all");
const API_MEWE_CONTACTS_FAVORITES: &str = concat!(api_mewe!(), "/v2/mycontacts/closefriends");
const API_MEWE_GROUPS: &str = concat!(api_mewe!(), "/v2/groups");
const API_MEWE_GROUP_INFO: &str = concat!(api_mewe!(), "/v2/group/{group_id}");


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
    pub fn new(cookies_path: &str) -> Option<MeweApi> {
        let cookies_path = cookies_path.to_string();
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

    pub fn get(&self, url: &str) -> reqwest::Result<Response> {
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
            .filter(|(n, _v)| n == "csrf-token")
            .map(|(_, v)| v)
            .take(1)
            .collect();

        if !csrf_token.is_empty() {
            self.headers.lock().unwrap()
                .insert("x-csrf-token".to_string(), csrf_token);
        }
        if cookies_len > 0 {
            // Если был какой либо set-cookie, сохраняем актуальный стор
            self.save_cookies(url);
        }
        if result.status() != 200 {
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
        if let Some(limit) = limit {
            let limit = limit.to_string();
            let query = HashMap::from([("limit", limit.as_str())]);
            update_query(&mut url, &query);
        }
        let response = self.get(url.as_str()).ok()?;
        if response.status() == 200 {
            Some(response.json::<json::MeweApiFeedList>().unwrap())
        } else {
            dbg!(&response.text());
            None
        }
    }

    // Todo iterator
    pub fn fetch_feeds(&self, url: &str, limit: Option<usize>, pages: Option<usize>)
                       -> Option<Vec<json::MeweApiFeedList>> {
        self.identify()?;
        let pages = pages.unwrap_or(1);

        let mut result: Vec<json::MeweApiFeedList> = Vec::with_capacity(pages);
        let mut next_page = url.to_string();
        for i in 0..pages {
            if i > 0 {
                // Не дрочим
                thread::sleep(Duration::from_millis(100));
            }
            let json = self.fetch_feed(next_page.as_str(), limit)?;
            if let Some(MeweApiFeedListNextPageLink { next_page: Some(page) }) = &json.links {
                next_page = page.href.clone();
            }
            result.push(json)
        }

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }

    pub fn get_my_feeds(&self, limit: Option<usize>, pages: Option<usize>) -> Option<Vec<json::MeweApiFeedList>> {
        self.fetch_feeds(API_MEWE_ALLFEED, limit, pages)
    }
    pub fn get_user_feed(&self, user_id: &str, limit: Option<usize>, pages: Option<usize>) -> Option<Vec<json::MeweApiFeedList>> {
        self.fetch_feeds(
            API_MEWE_USER_FEED.replace("{user_id}", user_id).as_str(),
            limit, pages)
    }
    pub fn get_group_feed(&self, group_id: &str, limit: Option<usize>, pages: Option<usize>) -> Option<Vec<json::MeweApiFeedList>> {
        self.fetch_feeds(
            API_MEWE_GROUP_FEED.replace("{group_id}", group_id).as_str(),
            limit, pages)
    }
    pub fn fetch_groups(&self) -> Option<json::MeweApiGroupList> {
        let response = self.get(API_MEWE_GROUPS).ok()?;
        if response.status() == 200 {
            Some(response.json::<json::MeweApiGroupList>().unwrap())
        } else {
            None
        }
    }
    pub fn fetch_group_info(&self, group_id: &str) -> Option<json::MeweApiGroup> {
        let url = API_MEWE_GROUP_INFO.replace("{group_id}", group_id);
        let response = self.get(url.as_str()).ok()?;
        if response.status() == 200 {
            Some(response.json::<json::MeweApiGroup>().unwrap())
        } else {
            None
        }
    }

    pub fn fetch_contact_info(&self, invite_id: &str) -> Option<json::MeweApiContactUser> {
        let url = API_MEWE_CONTACT_INFO.replace("{invite_id}", invite_id);
        let response = self.get(url.as_str()).ok()?;
        if response.status() == 200 {
            Some(response.json::<json::MeweApiContactUser>().unwrap())
        } else {
            None
        }
    }


    pub fn fetch_contact_page(&self, url: &str, limit: usize, offset: Option<usize>) -> Option<json::MeweApiContactList> {
        let mut url = Url::parse(url).unwrap();
        url.query_pairs_mut().append_pair("maxResults", limit.to_string().as_str());
        if let Some(offset) = offset {
            url.query_pairs_mut().append_pair("offset", offset.to_string().as_str());
        }
        let response = self.get(url.as_str()).ok()?;

        if response.status() == 200 {
            Some(response.json::<json::MeweApiContactList>().unwrap())
        } else {
            None
        }
    }
    pub fn fetch_contacts(&self, url: &str, limit: Option<usize>, pages: Option<usize>) -> Option<Vec<json::MeweApiContactUser>> {
        let pages = pages.unwrap_or(20);
        let limit = limit.unwrap_or(21);
        let mut res: Vec<json::MeweApiContactUser> = Vec::with_capacity(limit * pages);
        for i in 0..pages {
            let offset = if i == 0 { None } else { Some(i * limit) };
            let json = self.fetch_contact_page(url, limit, offset);
            if let Some(json) = json {
                if json.contacts.len() == 0 {
                    break;
                }
                res.extend(json.contacts.iter().map(|c| c.user.clone()));
            } else {
                return None;
            }
        }
        res.shrink_to_fit();
        Some(res)
    }

    pub fn get_contacts(&self, favorites: bool) -> Option<Vec<json::MeweApiContactUser>> {
        let url = if favorites {
            API_MEWE_CONTACTS_FAVORITES
        } else {
            API_MEWE_CONTACTS_ALL
        };
        self.fetch_contacts(url, None, None)
    }
}


#[cfg(test)]
mod test {
    use crate::MeweApi;

    const COOKIE_PATH: &str = "/home/apkawa/Downloads/mewe.com_cookies.txt";

    #[test]
    fn test_identify() {
        let mut mewe = MeweApi::new(COOKIE_PATH).unwrap();
        dbg!(&mewe);
        let info = mewe.whoami().unwrap();
        assert_eq!(info.contact_invite_id, "apkawa");
    }

    #[test]
    fn test_get_feeds() {
        let mewe = MeweApi::new(COOKIE_PATH).unwrap();
        let feeds = mewe.get_my_feeds(None, None).unwrap();
        dbg!(&feeds);
    }

    #[test]
    fn test_get_user_feed() {
        let mewe = MeweApi::new(COOKIE_PATH).unwrap();
        let contacts = mewe.get_contacts(true).unwrap();
        let feeds = mewe.get_user_feed(contacts[0].id.as_str(), None, None).unwrap();
        dbg!(&feeds);
    }

    #[test]
    fn test_get_group_feed() {
        let mewe = MeweApi::new(COOKIE_PATH).unwrap();
        let groups = mewe.fetch_groups().unwrap();
        let feeds = mewe.get_group_feed(groups.confirmed_groups[0].id.as_str(), None, None).unwrap();
        dbg!(&feeds);
    }


    #[test]
    fn test_fetch_groups() {
        let mewe = MeweApi::new(COOKIE_PATH).unwrap();
        let feeds = mewe.fetch_groups().unwrap();
        dbg!(&feeds);
    }

    #[test]
    fn test_fetch_contacts() {
        let mewe = MeweApi::new(COOKIE_PATH).unwrap();
        let contacts = mewe.get_contacts(true).unwrap();
        dbg!(&contacts);
    }
}
