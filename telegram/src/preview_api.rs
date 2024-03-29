use std::sync::Arc;

use reqwest::blocking::Response;
use reqwest::cookie::Jar;
use reqwest::Method;
use scraper;
use scraper::Selector;

use crate::data::Channel;
use crate::error;
use crate::error::TelegramApiError;
use crate::parse::parse_message;
use crate::TelegramApiErrorKind::StatusError;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

#[derive(Default, Debug)]
pub struct TelegramChannelPreviewApi {
    session: reqwest::blocking::Client,
    pub slug: String,
    pub name: Option<String>,
}

impl TelegramChannelPreviewApi {
    pub fn new(slug: &str) -> TelegramChannelPreviewApi {
        let jar = Jar::default();
        let session = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .cookie_provider(Arc::new(jar))
            .build()
            .unwrap();

        TelegramChannelPreviewApi {
            slug: slug.to_string(),
            session,
            ..TelegramChannelPreviewApi::default()
        }
    }

    pub fn preview_url(&self) -> String {
        format!("https://t.me/s/{}", self.slug)
    }
    pub fn embedded_post_url(&self, id: usize) -> String {
        format!(
            "https://t.me/{}/{}?embed=1&mode=tme&userpic=true",
            self.slug, id
        )
    }

    pub fn request(&self, method: Method, url: &str) -> reqwest::blocking::RequestBuilder {
        self.session.request(method, url)
    }

    pub fn get(&self, url: &str) -> crate::Result<Response> {
        log::debug!("get url={:?}", url);
        let result = self.request(Method::GET, url).send()?;
        let status = result.status().as_u16();
        log::debug!("get [{:?}]", status);
        log::trace!("get result={:?}", &result);
        if status >= 400 {
            log::trace!("ERROR text={:?}", &result.text());
            Err(TelegramApiError::ApiError {
                kind: StatusError(status),
            })
        } else {
            Ok(result)
        }
    }

    pub fn parse_html_page(&self, html: &str) -> error::Result<Channel> {
        let parser = scraper::Html::parse_document(html);
        let mut channel = Channel {
            slug: self.slug.clone(),
            ..Channel::default()
        };
        for el in parser.select(&Selector::parse("meta[property^='og:']").unwrap()) {
            let v = el.value();
            let prop = v.attr("property").unwrap();
            match prop[3..].as_ref() {
                "title" => channel.title = v.attr("content").unwrap().to_string(),
                "image" => channel.image_url = v.attr("content").unwrap().to_string(),
                "description" => channel.description = v.attr("content").unwrap().to_string(),
                _ => {}
            }
        }
        log::trace!("parsed meta {:?}", channel);
        for el_ref in parser.select(&Selector::parse(".js-widget_message").unwrap()) {
            log::debug!(
                "start parse message id={:?}",
                el_ref.value().attr("data-post")
            );
            let post = parse_message(el_ref.html().as_str()).unwrap();
            log::trace!("parsed message {:?}", post);
            channel.posts.push(post);
        }
        Ok(channel)
    }
    /// Пытаемся получить новый урл.
    pub fn try_get_new_media_url(&self, post_id: usize, media_index: usize, field: &str) -> String {
        log::trace!(
            "try_get_new_media_url post_id={:?} media_index={:?} field={:?}",
            post_id,
            media_index,
            field
        );
        let channel = self.fetch_post(post_id).unwrap();
        let post = channel.posts.get(0).unwrap();
        let url = post.media_try_get_new_url(media_index, field);
        log::trace!("try_get_new_media_url url={:?}", url);
        url
    }

    pub fn fetch_post(&self, id: usize) -> error::Result<Channel> {
        log::debug!("fetch_post id={:?}", id);
        let html = self.get(self.embedded_post_url(id).as_str())?.text()?;
        self.parse_html_page(html.as_str())
    }

    pub fn fetch(&self, _pages: Option<usize>) -> error::Result<Channel> {
        log::debug!("fetch pages={:?}", _pages);
        // TODO handle pages
        let html = self.get(self.preview_url().as_str())?.text()?;
        self.parse_html_page(html.as_str())
    }
}

#[cfg(test)]
mod test {
    use reqwest::Url;

    use crate::preview_api::TelegramChannelPreviewApi;

    #[test]
    fn test_preview_api() {
        let api = TelegramChannelPreviewApi::new("fighter_bomber");
        let channel = api.fetch(None).unwrap();
        dbg!(&channel);
        assert!(!channel.posts.is_empty());
    }

    #[test]
    fn test_embedded_post() {
        let api = TelegramChannelPreviewApi::new("fighter_bomber");
        let channel = api.fetch_post(11245).unwrap();
        dbg!(&channel);
        assert_eq!(channel.posts.len(), 1);
    }

    #[test]
    fn test_try_get_photo_media_url() {
        let api = TelegramChannelPreviewApi::new("fighter_bomber");
        let new_url = api.try_get_new_media_url(11247, 0, "url");
        dbg!(&new_url);
        assert!(&new_url.ends_with(".jpg"));
    }

    #[test]
    fn test_try_get_media_video_url() {
        let api = TelegramChannelPreviewApi::new("fighter_bomber");
        let channel = api.fetch_post(11241).unwrap();
        let post = channel.posts.get(0).unwrap();

        // Try get video
        let new_url = post.media_try_get_new_url(0, "url");
        dbg!(&new_url);
        let new_url = Url::parse(&new_url).unwrap();
        assert!(&new_url.path().ends_with(".mp4"));

        // Try get thumbnail
        let new_url = post.media_try_get_new_url(0, "thumb_url");
        dbg!(&new_url);
    }

    #[test]
    fn test_try_get_video_too_big() {
        let api = TelegramChannelPreviewApi::new("fighter_bomber");
        let new_url = api.try_get_new_media_url(11246, 0, "thumb_url");
        dbg!(&new_url);
    }
}
