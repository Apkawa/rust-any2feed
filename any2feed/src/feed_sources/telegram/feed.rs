use crate::feed_sources::traits::RenderContent;
use crate::feed_sources::utils::timestamp_now;
use chrono::Local;
use feed::{CDATAElement, Content, Element, Entry, Feed, Link, Person};
use reqwest::Url;
use telegram::data::{Channel, ChannelPost, Media};

pub struct Context {
    pub proxy_url: Url,
}

/// Меняем у поста все медиа ссылки на прокси перед отрисовкой
pub fn set_proxy_url(mut post: ChannelPost, proxy_url: &Url) -> ChannelPost {
    let mut url = proxy_url.clone();
    url.path_segments_mut().unwrap().extend(post.id.split('/'));
    url.query_pairs_mut()
        .append_pair("t", timestamp_now().to_string().as_str());
    let mut medias = post.get_media_list_mut();
    for (i, media) in medias.iter_mut().enumerate() {
        media_set_proxy_urls(&url, media, i);
    }
    return post;

    fn media_set_proxy_urls(proxy_url: &Url, media: &mut Media, i: usize) {
        use Media::*;
        match media {
            Photo(ref mut url) | Voice(ref mut url) => {
                *url = build_media_url(proxy_url.clone(), url, i, "url").to_string();
            }
            Video {
                ref mut url,
                ref mut thumb_url,
            }
            | VideoGif {
                ref mut url,
                ref mut thumb_url,
            } => {
                *url = build_media_url(proxy_url.clone(), url, i, "url").to_string();
                *thumb_url =
                    build_media_url(proxy_url.clone(), thumb_url, i, "thumb_url").to_string();
            }
            VideoTooBig { ref mut thumb_url } => {
                *thumb_url =
                    build_media_url(proxy_url.clone(), thumb_url, i, "thumb_url").to_string();
            }
        }
    }

    fn build_media_url(mut proxy_url: Url, url: &str, i: usize, prefix: &str) -> Url {
        proxy_url.query_pairs_mut().append_pair("url", url);
        proxy_url
            .path_segments_mut()
            .unwrap()
            .push(format!("{i}-{prefix}").as_str())
            .push("");

        proxy_url
    }
}

pub fn channel_post_to_entry(post: ChannelPost, context: Option<&Context>) -> Entry {
    let title = post
        .text
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join(" ");

    let mut entry = Entry::new(post.id.clone(), title, post.datetime.clone());

    entry.link = Some(Link::new(post.preview_url()));
    let post = if let Some(Context { proxy_url }) = context {
        set_proxy_url(post, proxy_url)
    } else {
        post
    };

    entry.content = Some(Content::Html(post.render().unwrap()));

    // TODO автора поста в канале лучше отображать где нибудь незаметно в тексте
    // entry.author = Element(Person::new(from_author.clone(), None, None))

    entry
}

pub fn channel_to_feed(channel: &Channel, context: Option<&Context>) -> Feed {
    let mut feed = Feed {
        title: CDATAElement(channel.title.clone()),
        updated: Local::now().to_rfc3339(),
        author: Element(Person::new(
            channel.title.clone(),
            Some(channel.preview_url()),
            None,
        )),
        // todo logo
        subtitle: Some(Content::Text(channel.description.clone())),
        ..Feed::default()
    };
    feed.link.push(Link::new(channel.preview_url()));
    feed.entries = channel
        .posts
        .iter()
        .map(|p| {
            let mut e = channel_post_to_entry(p.clone(), context);
            e.author = Element(Person::new(
                channel.title.clone(),
                Some(channel.preview_url()),
                None,
            ));
            e
        })
        .collect();
    feed
}
