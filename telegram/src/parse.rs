use crate::data::{ChannelPost, File, ForwardedFrom, LinkPreview, Media, Poll, PollOption};
use regex::Regex;
use scraper::node::Element;
use scraper::Selector;

///
/// ```
/// use scraper::{ElementRef, Html, Selector};
/// use telegram::parse::get_class_name_by_prefix;
/// let h = Html::parse_fragment(r#"<div class="foo-bar js-message_photo js-foo-bar"></div>"#);
/// let pr = ".js-message_";
/// let s = Selector::parse(format!("{pr}photo").as_str()).unwrap();
/// let el_ref: ElementRef = h.select(&s).next().unwrap();
///
/// assert_eq!(get_class_name_by_prefix(el_ref.value(), pr), Some("photo"));
/// assert_eq!(get_class_name_by_prefix(el_ref.value(), "foo-"), Some("bar"));
/// assert_eq!(get_class_name_by_prefix(el_ref.value(), "lalala"), None);
/// ```
pub fn get_class_name_by_prefix<'a>(el: &'a Element, prefix: &str) -> Option<&'a str> {
    let prefix = prefix.trim_start_matches('.');
    el.classes()
        .find(|p| p.starts_with(prefix))
        .map(|s| s.trim_start_matches(prefix))
}

/// ```
///
/// use scraper::{ElementRef, Html, Selector};
/// use telegram::parse::has_class;
/// let h = Html::parse_fragment(r#"<div class="foo-bar js-message_photo js-foo-bar"></div>"#);
/// let s = Selector::parse("div").unwrap();
/// let el_ref: ElementRef = h.select(&s).next().unwrap();
///
/// assert!(has_class(el_ref.value(), "foo-bar"));
/// assert!(!has_class(el_ref.value(), "foo"));
/// ```
pub fn has_class(el: &Element, class: &str) -> bool {
    el.classes().any(|c| c.eq(class))
}

///
/// ```
/// use telegram::parse::get_background_url_from_style;
/// let style = r#"width:718px;background-image:url('https://cdn4.telegram-cdn.org/file/A..Y.jpg')"#;
///
/// assert_eq!(get_background_url_from_style(style), Some("https://cdn4.telegram-cdn.org/file/A..Y.jpg"));
/// assert_eq!(get_background_url_from_style("Foobar"), None)
/// ```
pub fn get_background_url_from_style(style: &str) -> Option<&str> {
    let re = Regex::new(r#"background-image\s*:\s*url\(['"](.+?)['"]\)"#).unwrap();
    re.captures(style).map(|c| c.get(1).unwrap().as_str())
}

pub fn parse_media_video(html: &str) -> Media {
    let mut video_url: Option<String> = None;
    let mut thumb_url: Option<String> = None;
    let mut is_gif = false;
    let parser = scraper::Html::parse_fragment(html);
    let cls_prefix = ".tgme_widget_message_";
    let selector = Selector::parse(
        format!(
            "{p}video, {p}video_thumb, {p}roundvideo, {p}roundvideo_thumb",
            p = cls_prefix
        )
        .as_str(),
    )
    .unwrap();
    for el_ref in parser.select(&selector) {
        let el = el_ref.value();
        match get_class_name_by_prefix(el, cls_prefix) {
            Some(t @ ("video" | "roundvideo")) => {
                video_url = Some(el.attr("src").unwrap().to_string());
                is_gif = t == "video" && el.attr("autoplay").is_some() && el.attr("loop").is_some();
            }
            Some("video_thumb" | "roundvideo_thumb") => {
                thumb_url = Some(
                    get_background_url_from_style(el.attr("style").unwrap())
                        .unwrap()
                        .to_string(),
                )
            }
            _ => unreachable!(),
        }
    }
    match (video_url, thumb_url) {
        (Some(v), Some(t)) => {
            if is_gif {
                Media::VideoGif {
                    url: v,
                    thumb_url: t,
                }
            } else {
                Media::Video {
                    url: v,
                    thumb_url: t,
                }
            }
        }
        (None, Some(t)) => Media::VideoTooBig { thumb_url: t },
        _ => {
            println!("{}", html);
            unreachable!()
        }
    }
}

pub fn parse_link_preview(html: &str) -> LinkPreview {
    let mut link = LinkPreview::default();
    let parser = scraper::Html::parse_fragment(html);
    let selector = Selector::parse(
        ".tgme_widget_message_link_preview, \
             .link_preview_site_name, \
             .link_preview_image, \
             .link_preview_video_thumb, \
             .link_preview_video, \
             .link_preview_title, \
             .link_preview_description",
    )
    .unwrap();
    let mut image_url: Option<String> = None;
    let mut video_url: Option<String> = None;
    let mut video_thumb_url: Option<String> = None;

    for el_ref in parser.select(&selector) {
        let el = el_ref.value();
        if has_class(el, "tgme_widget_message_link_preview") {
            link.url = el.attr("href").unwrap().to_string();
        }
        match get_class_name_by_prefix(el, "link_preview_") {
            Some("site_name") => link.site_name = el_ref.inner_html(),
            Some("title") => link.title = el_ref.inner_html(),
            Some("description") => link.description = el_ref.inner_html(),
            Some("image") => {
                image_url =
                    get_background_url_from_style(el.attr("style").unwrap()).map(|s| s.to_string())
            }
            Some("video") => video_url = el.attr("src").map(|s| s.to_string()),
            Some("video_thumb") => {
                video_thumb_url =
                    get_background_url_from_style(el.attr("style").unwrap()).map(|s| s.to_string())
            }

            _ => {}
        }
    }
    link.media = match (image_url, video_url, video_thumb_url) {
        (Some(url), None, None) => Some(Media::Photo(url)),
        // TODO check gif
        (None, Some(url), Some(thumb_url)) => Some(Media::Video { url, thumb_url }),
        (None, None, Some(thumb_url)) => Some(Media::VideoTooBig { thumb_url }),
        _ => None,
    };
    link
}

pub fn parse_poll(html: &str) -> Poll {
    let mut poll = Poll::default();
    let parser = scraper::Html::parse_fragment(html);
    let selector = Selector::parse(
        ".tgme_widget_message_poll_question, \
        .tgme_widget_message_poll_type, \
        .tgme_widget_message_poll_option \
    ",
    )
    .unwrap();
    let option_percent_sel = Selector::parse(".tgme_widget_message_poll_option_percent").unwrap();
    let option_val_sel = Selector::parse(".tgme_widget_message_poll_option_text").unwrap();

    for el_ref in parser.select(&selector) {
        let el = el_ref.value();
        match get_class_name_by_prefix(el, "tgme_widget_message_poll_") {
            Some("question") => poll.question = el_ref.inner_html(),
            Some("type") => poll.r#type = el_ref.inner_html(),
            Some("option") => {
                let o = PollOption {
                    name: el_ref.select(&option_val_sel).next().unwrap().inner_html(),
                    percent: el_ref
                        .select(&option_percent_sel)
                        .next()
                        .unwrap()
                        .inner_html(),
                };
                poll.options.push(o)
            }
            _ => {
                dbg!(el);
                unreachable!()
            }
        }
    }

    poll
}

pub fn parse_message(html: &str) -> Option<ChannelPost> {
    let mut post = ChannelPost::default();
    let class_prefix = ".js-message_";

    let parser = scraper::Html::parse_fragment(html);
    let selector = Selector::parse(
        format!(
            "{p}text, \
             {p}video_player, \
             {p}roundvideo_player, \
             .js-widget_message, \
             .tgme_widget_message_photo_wrap, \
             .tgme_widget_message_link_preview, \
             .tgme_widget_message_forwarded_from_name, \
             .tgme_widget_message_document, \
             .tgme_widget_message_poll, \
             .tgme_widget_message_voice, \
             time.time \
             ",
            p = class_prefix
        )
        .as_str(),
    )
    .unwrap();
    let document_title_sel = Selector::parse(".tgme_widget_message_document_title").unwrap();
    let document_extra_sel = Selector::parse(".tgme_widget_message_document_extra").unwrap();

    for el_ref in parser.select(&selector) {
        let el = el_ref.value();
        if el.name() == "time" {
            post.datetime = el.attr("datetime").unwrap().to_string();
        }
        match get_class_name_by_prefix(el, "tgme_widget_message_") {
            Some("photo_wrap") => {
                if let Some(photo) = el
                    .attr("style")
                    .and_then(get_background_url_from_style)
                {
                    post.media
                        .get_or_insert_with(Vec::new)
                        .push(Media::Photo(photo.to_string()))
                }
            }
            Some("link_preview") => {
                post.link_preview = Some(parse_link_preview(el_ref.html().as_str()))
            }
            Some("forwarded_from_name") => {
                post.forwarded_from = Some(ForwardedFrom {
                    name: el_ref.text().into_iter().collect(),
                    url: el.attr("href").unwrap().to_string(),
                })
            }
            Some("document") => post.file.get_or_insert_with(Vec::new).push(File {
                filename: el_ref
                    .select(&document_title_sel)
                    .next()
                    .unwrap()
                    .inner_html(),
                size: el_ref
                    .select(&document_extra_sel)
                    .next()
                    .unwrap()
                    .inner_html(),
            }),
            Some("voice") => {
                if let Some(src) = el.attr("src") {
                    post.media
                        .get_or_insert_with(Vec::new)
                        .push(Media::Voice(src.to_string()))
                }
            }
            _ => {}
        }
        match get_class_name_by_prefix(el, "js-") {
            Some("widget_message") => post.id = el.attr("data-post").unwrap().to_string(),
            Some("message_text") => {
                post.text = el_ref.text().into_iter().collect();
                post.html = el_ref.inner_html();
            }
            Some("message_video_player" | "message_roundvideo_player") => {
                if !has_class(el, "link_preview_video_player") {
                    post.media
                        .get_or_insert_with(Vec::new)
                        .push(parse_media_video(el_ref.html().as_str()))
                }
            }
            Some("poll") => post.poll = Some(parse_poll(el_ref.html().as_str())),
            _ => {}
        }
    }
    Some(post)
}
