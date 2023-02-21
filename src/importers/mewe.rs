use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use reqwest::{cookie::Jar, Url};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

use crate::http_client::cookie::{import_cookie_from_file, update_cookie_from_file};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

macro_rules! api_mewe {
    () => ( "https://mewe.com/api" )
}
const API_MEWE_IDENTIFY: &str = concat!(api_mewe!(), "/v3/auth/identify");
const API_MEWE_ME_INFO: &str = concat!(api_mewe!(), "/v2/me/info");

#[derive(Debug, Deserialize)]
struct MeweApiIdentify {
    authenticated: bool,
    confirmed: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MeweApiMeInfo {
    id: String,
    first_name: String,
    last_name: String,
    contact_invite_id: String,
    primary_email: String,

}

#[derive(Debug, Default)]
struct MeweApi {
    cookies_path: String,
    cookies: Arc<Jar>,
    session: reqwest::blocking::Client,
    headers: RefCell<HashMap<String, String>>,
    pub me_info: Option<MeweApiMeInfo>,
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
        for (k, v) in self.headers.borrow().iter() {
            rb = rb.header(k, v);
        }
        let result = rb.send()?;
        let csrf_token: String = result.cookies()
            .filter(|c| c.name() == "csrf-token")
            .map(|c| c.value().to_string())
            .take(1)
            .collect();

        if !csrf_token.is_empty() {
            self.headers.borrow_mut()
                .insert("x-csrf-token".to_string(), csrf_token);
        }
        self.save_cookies(&url);
        Ok(result)
    }

    pub fn whoami(&mut self) -> Option<MeweApiMeInfo> {
        let info: MeweApiMeInfo = self.get(API_MEWE_ME_INFO).ok()?.json().ok()?;
        self.me_info = Some(info.clone());
        Some(info)
    }


    pub fn identify(&self) -> Option<()> {
        let result = self.get(API_MEWE_IDENTIFY).ok()?;
        if result.status() == 200 {
            let json = {
                result.json::<MeweApiIdentify>().ok()?
            };
            if json.authenticated {
                return Some(());
            }
        }
        None
    }
}


#[cfg(test)]
mod test {
    use std::sync::Arc;

    use reqwest::cookie::Jar;
    use reqwest::Url;

    use crate::importers::mewe::{MeweApi, USER_AGENT};

    #[test]
    fn test_identify() {
        let mut mewe = MeweApi::new(
            "/home/apkawa/Downloads/mewe.com_cookies.txt".to_string()).unwrap();
        dbg!(&mewe);
        let info = mewe.whoami().unwrap();
        assert_eq!(info.contact_invite_id, "apkawa");

    }

    #[test]
    fn example_2() {
    }
}