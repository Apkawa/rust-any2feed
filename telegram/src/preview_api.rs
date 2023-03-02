use crate::data::Channel;
use crate::parse::parse_message;
use reqwest::blocking::Response;
use reqwest::cookie::Jar;
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

    pub fn get(&self, url: &str) -> reqwest::Result<Response> {
        let rb = self.session.get(url).send();
        rb
    }

    pub fn parse_html_page(&self, html: &str) -> Channel {
        let parser = scraper::Html::parse_document(html);
        let mut channel = Channel::default();
        channel.slug = self.slug.clone();
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
        for el_ref in parser.select(&Selector::parse(".js-widget_message_wrap").unwrap()) {
            channel
                .posts
                .push(parse_message(el_ref.html().as_str()).unwrap());
        }

        channel
    }

    pub fn fetch(&self) -> reqwest::Result<Channel> {
        let html = self.get(self.preview_url().as_str())?.text()?;
        Ok(self.parse_html_page(html.as_str()))
    }
}

#[cfg(test)]
mod test {
    use crate::preview_api::TelegramChannelPreviewApi;

    #[test]
    fn test_preview_api() {
        let api = TelegramChannelPreviewApi::new("fighter_bomber");
        let channel = api.fetch().unwrap();
        dbg!(&channel);
    }
}
