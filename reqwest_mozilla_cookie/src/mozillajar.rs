// TODO Выделить в крейт reqwest-cookie-mozillajar https://docs.python.org/3/library/http.cookiejar.html#filecookiejar-subclasses-and-co-operation-with-web-browsers


use std::sync::RwLock;
use cookie_store;
use cookie_crate;
use reqwest::cookie::{Cookie, CookieStore};
use reqwest::header::HeaderValue;
use reqwest::{cookie, Url};

struct MozillaJar {
    path: String,
    store: RwLock<cookie_store::CookieStore>,
}

impl MozillaJar {
    /// Add a cookie to this jar.
    ///
    /// # Example
    ///
    /// ```
    /// ```
    pub fn add_cookie_str(&self, cookie: &str, url: &Url) {
        let cookies = cookie_crate::Cookie::parse(cookie)
            .ok()
            .map(|c| c.into_owned())
            .into_iter();
        self.store.write().unwrap().store_response_cookies(cookies, url);
    }
}

impl CookieStore for MozillaJar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item=&HeaderValue>, url: &Url) {
        let iter = cookie_headers.filter_map(
            |val|
                cookie_crate::Cookie::parse(val)
                .map(|c| c.0.into_owned()).ok()
        );

        self.store.write().unwrap().store_response_cookies(iter, url);
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let s = self
            .store
            .read()
            .unwrap()
            .get_request_values(url)
            .map(|(name, value)| format!("{}={}", name, value))
            .collect::<Vec<_>>()
            .join("; ");

        if s.is_empty() {
            return None;
        }

        HeaderValue::from_maybe_shared(Bytes::from(s)).ok()
    }
}

