
use telegram::data::Media;
use telegram::parse::{parse_link_preview, parse_media_video, parse_message};
use telegram::preview_api::TelegramChannelPreviewApi;
use test_utils::fixture::load_fixture;

#[test]
fn test_parse_html() {
    let api = TelegramChannelPreviewApi::new("foo");
    let html = load_fixture("telegram_preview/full.html");
    let result = api.parse_html_page(html.as_str());
    assert_eq!(result.slug, "foo");
    assert_eq!(result.title, "Военное обозрение");
    assert!(result.description.starts_with("Официальный канал сайта"));
    assert!(result.image_url.starts_with("https://cdn4.telegram-cdn.org/file/"));
    assert!(result.posts.len() > 0);
}


#[test]
fn test_parse_text() {
    let html = load_fixture("telegram_preview/message_text.html");
    let result = parse_message(html.as_str()).unwrap();
    assert!(&result.text.starts_with("<b>Неопознанный летающий объект"));
}

#[test]
fn test_parse_link_preview() {
    let html = load_fixture("telegram_preview/link_preview.html");
    let link = parse_link_preview(html.as_str());
    dbg!(&link);
    assert!(!link.title.is_empty());
    assert!(!link.description.is_empty());
    assert!(!link.url.is_empty());
    assert!(!link.site_name.is_empty());
    assert!(link.image_url.is_some());

}

#[test]
fn test_parse_media_video() {
    let html = load_fixture("telegram_preview/media_video.html");
    if let Media::Video { url, thumb_url } = parse_media_video(html.as_str()) {
        assert!(url.starts_with("https://cdn4.telegram-cdn.org/"));
        assert!(thumb_url.starts_with("https://cdn4.telegram-cdn.org/"));
    } else {
        unreachable!();
    }
}

#[test]
fn test_parse_media_video_too_big() {
    let html = load_fixture("telegram_preview/media_video_too_big.html");
    if let Media::VideoTooBig { thumb_url } = parse_media_video(html.as_str()) {
        assert!(thumb_url.starts_with("https://cdn4.telegram-cdn.org/"));
    } else {
        unreachable!();
    }
}

#[test]
fn test_parse_circle_video() {
    let html = load_fixture("telegram_preview/media_cicrle_video.html");
    if let Media::Video { url, thumb_url } = parse_media_video(html.as_str()) {
        assert!(url.starts_with("https://cdn4.telegram-cdn.org/"));
        assert!(thumb_url.starts_with("https://cdn4.telegram-cdn.org/"));
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod parametrize {
    use rstest::rstest;
    use telegram::parse::parse_message;
    use test_utils::fixture::load_fixture;

    #[rstest]
    #[case("text")]
    // TODO media file
    // #[case("media_file")]
    // TODO poll
    // #[case("poll")]
    #[case("media_link_preview")]
    #[case("media_link_preview_text_only")]
    #[case("media_forwarded_from")]
    #[case("media_photo_and_video")]
    #[case("link_preview")]
    fn test_message(#[case] name: &str) {
        let html = load_fixture(format!("telegram_preview/message_{name}.html").as_str());
        let result = parse_message(html.as_str()).unwrap();
        assert!(!result.id.is_empty());
        assert!(!result.datetime.is_empty());
        assert!(!result.text.is_empty());
        dbg!(&result);
    }
}