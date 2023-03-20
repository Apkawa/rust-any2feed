use any2feed::feed_sources::telegram::feed::{channel_post_to_entry, set_proxy_url, Context};
use feed::Content;
use reqwest::Url;
use std::collections::HashMap;
use telegram::data::{ChannelPost, LinkPreview, Media};
use telegram::parse::parse_message;
use test_utils::fixture::load_fixture;

#[test]
fn test_set_proxy_url() {
    let mut post = ChannelPost {
        id: "channel_name/123".to_string(),
        media: Some(vec![
            Media::Photo("https://url.com/1/".to_string()),
            Media::Photo("https://url.com/2/".to_string()),
            Media::Video {
                url: "https://url.com/3/".to_string(),
                thumb_url: "https://url.com/3/".to_string(),
            },
        ]),
        link_preview: Some(LinkPreview {
            media: Some(Media::Photo("https://url.com/link_preview/".to_string())),
            ..LinkPreview::default()
        }),
        ..ChannelPost::default()
    };

    assert_eq!(post.get_media_list_mut().len(), 4);
    assert_eq!(post.get_media_list().len(), 4);
    assert_eq!(
        post.get_media_list()[3].get_urls(),
        vec!["https://url.com/link_preview/".to_string()]
    );
    assert_eq!(
        post.get_media(3).unwrap().get_urls(),
        vec!["https://url.com/link_preview/".to_string()]
    );

    let post = set_proxy_url(post, &Url::parse("http://localhost.com/").unwrap());

    assert_eq!(post.get_media_list().len(), 4);
    let url = Url::parse(post.get_media_list()[3].get_urls()[0].as_str()).unwrap();
    assert!(url
        .as_str()
        .starts_with("http://localhost.com/channel_name/123/3-url/"));
    let query_params: HashMap<String, String> = url
        .query_pairs()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    assert!(query_params.contains_key("t"));
    assert_eq!(
        query_params.get("url"),
        Some(&"https://url.com/link_preview/".to_string())
    )
}

fn load_channel_post_fixture(name: &str) -> ChannelPost {
    let html = load_fixture(format!("telegram_preview/message_{name}.html").as_str());
    parse_message(html.as_str()).unwrap()
}

#[test]
fn test_without_proxy_replace() {
    let p = load_channel_post_fixture("media_photo_and_video");
    let e = channel_post_to_entry(p, None);
    dbg!(&e);
    let Some(Content::Html(ref content)) = e.content else { unreachable!() };
    assert!(content.contains(r#"poster="https://cdn4.telegram-cdn.org/file/"#))
}

#[test]
fn test_proxy_replace() {
    let p = load_channel_post_fixture("media_photo_and_video");
    let context = Context {
        proxy_url: Url::parse("http://localhost:12345/telegram/media").unwrap(),
    };
    let e = channel_post_to_entry(p, Some(&context));
    dbg!(&e);
    let Some(Content::Html(ref content)) = e.content else { unreachable!() };
    assert!(content.contains(
        r#"poster="http://localhost:12345/telegram/media/anna_news/46908/0-thumb_url/?t="#
    ))
}
