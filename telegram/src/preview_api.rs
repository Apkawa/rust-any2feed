use crate::data::Channel;
use crate::parse::parse_message;
use reqwest::blocking::Response;
use reqwest::cookie::Jar;
use reqwest::Method;
use scraper;
use scraper::Selector;
use std::sync::Arc;

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

    pub fn get(&self, url: &str) -> reqwest::Result<Response> {
        self.request(Method::GET, url).send()
    }

    pub fn parse_html_page(&self, html: &str) -> Channel {
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
        for el_ref in parser.select(&Selector::parse(".js-widget_message").unwrap()) {
            channel
                .posts
                .push(parse_message(el_ref.html().as_str()).unwrap());
        }

        channel
    }
    /// Пытаемся получить новый урл.
    pub fn try_get_new_media_url(&self, post_id: usize, media_index: usize, field: &str) -> String {
        let channel = self.fetch_post(post_id).unwrap();
        let post = channel.posts.get(0).unwrap();
        post.media_try_get_new_url(media_index, field)
    }

    pub fn fetch_post(&self, id: usize) -> reqwest::Result<Channel> {
        let html = self.get(self.embedded_post_url(id).as_str())?.text()?;
        Ok(self.parse_html_page(html.as_str()))
    }

    pub fn fetch(&self, _pages: Option<usize>) -> reqwest::Result<Channel> {
        // TODO handle pages
        let html = self.get(self.preview_url().as_str())?.text()?;
        Ok(self.parse_html_page(html.as_str()))
    }
}

#[cfg(test)]
mod test {
    use crate::preview_api::TelegramChannelPreviewApi;
    use reqwest::Url;

    #[test]
    fn test_preview_api() {
        let api = TelegramChannelPreviewApi::new("fighter_bomber");
        let channel = api.fetch(None).unwrap();
        dbg!(&channel);
        assert!(channel.posts.len() > 0);
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
