use telegram::data::Media;
use telegram::parse::{parse_link_preview, parse_media_video, parse_message, parse_poll};
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
    assert!(result
        .image_url
        .starts_with("https://cdn4.telegram-cdn.org/file/"));
    assert!(!result.posts.is_empty());
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
    assert!(link.media.is_some());
}

#[test]
fn test_parse_link_preview_video() {
    let html = load_fixture("telegram_preview/link_preview_video.html");
    let link = parse_link_preview(html.as_str());
    dbg!(&link);
    assert!(!link.title.is_empty());
    assert!(!link.description.is_empty());
    assert!(!link.url.is_empty());
    assert!(!link.site_name.is_empty());
    assert!(link.media.is_some());
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
fn test_parse_media_video_gif() {
    let html = load_fixture("telegram_preview/media_video_gif.html");
    if let Media::VideoGif { url, thumb_url } = parse_media_video(html.as_str()) {
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

#[test]
fn test_parse_poll() {
    let html = load_fixture("telegram_preview/poll.html");
    let poll = parse_poll(html.as_str());
    dbg!(&poll);
}

#[test]
fn test_parse_message_poll() {
    let html = load_fixture("telegram_preview/message_poll.html");
    let post = parse_message(html.as_str()).unwrap();
    dbg!(&post);
    assert!(post.poll.is_some());
}

#[cfg(test)]
mod parametrize {
    use rstest::rstest;
    use telegram::parse::parse_message;
    use test_utils::fixture::load_fixture;

    #[rstest]
    #[case("poll")]
    #[case("text")]
    #[case("media_file")]
    // TODO sticker
    // #[case("sticker")]
    #[case("link_preview")]
    #[case("link_preview_text_only")]
    #[case("link_preview_video")]
    #[case("media_forwarded_from")]
    #[case("media_photo_and_video")]
    #[case("media_round_video")]
    #[case("link_preview")]
    fn test_message(#[case] name: &str) {
        let html = load_fixture(format!("telegram_preview/message_{name}.html").as_str());
        let result = parse_message(html.as_str()).unwrap();
        assert!(!result.id.is_empty());
        assert!(!result.datetime.is_empty());
        dbg!(&result);
    }
}
