use ::feed::Entry;
use booru_rs::client::generic::model::{Image, ImageSize};
use booru_rs::client::generic::BooruPostModel;
use chrono::Local;
use feed::{Category, Content, Element, Feed, Link, Person};
use reqwest::Url;
use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct Context {
    pub proxy_url: Option<Url>,
    pub key: Option<String>,
}

pub fn build_proxy_url(media_url: &str, mut proxy_url: Url) -> String {
    proxy_url.query_pairs_mut().append_pair("url", media_url);
    proxy_url.to_string()
}

pub fn danbooru_post_to_entry(post: Box<dyn BooruPostModel>, context: Option<&Context>) -> Entry {
    let sample = if post.images().sample.is_none() {
        String::new()
    } else {
        // let img = DanbooruImage::from_md5(&post.md5.unwrap(), &post.file_ext);
        let sample = post.images().sample.unwrap().url;
        let sample = if let Some(Context {
            proxy_url: Some(proxy_url),
            ..
        }) = context
        {
            build_proxy_url(&sample, proxy_url.clone())
        } else {
            sample.to_string()
        };

        format!(
            r#"
        <img
          src="{sample}"
          />
        "#,
            sample = sample,
        )
    };

    let full_img = if post.images().original.is_none() {
        String::new()
    } else {
        let Some(Image {
            url,
            filesize,
            size,
            ext }) = post.images().original else { unreachable!() };
        let mut title = String::new();
        if let Some(ext) = ext {
            title.push_str(format!(".{ext} ").as_str());
        }
        if let Some(ImageSize { width, height }) = size {
            title.push_str(format!("{width}x{height} ").as_str());
        }
        if let Some(filesize) = filesize {
            let filesize = filesize / 1024;
            title.push_str(format!("{filesize}KB").as_str());
        }

        format!(r#"<p><a href="{url}">Full {title}</a></p>"#)
    };

    let source = post
        .source_url()
        .unwrap_or_else(|| Cow::Owned(String::new()));

    let content = format!(
        r#"
        <p><a href="{source}">{source}</a></p>
        {full_img}
        {sample}
        "#,
    );
    let mut post_id = post.id().to_string();
    if let Some(Context { key: Some(key), .. }) = context {
        // For intersection feeds
        post_id.push_str(key)
    }
    let mut entry = Entry::new(
        post_id,
        post.tags().join(" "),
        post.created().unwrap().to_string(),
    );
    entry.published = post.created().map(|c| Element(c.to_string()));
    entry.content = Some(Content::Html(content));
    entry.link = Some(Link::new(post.post_url().unwrap().into()));
    let categories: Vec<Category> = post
        .tags()
        .iter()
        .map(|t| Category::new(t.to_string(), None, None))
        .collect();
    entry.categories = Some(Element(categories));
    if let Some(artist) = post.artist() {
        entry.author = Element(Person::new(artist.to_string(), None, None));
    }
    entry
}

pub fn booru_posts_to_feed(posts: Vec<Box<dyn BooruPostModel>>, context: Option<&Context>) -> Feed {
    let entry_list: Vec<Entry> = posts
        .into_iter()
        .map(|p| danbooru_post_to_entry(p, context))
        .collect();

    let mut feed = Feed {
        updated: Local::now().to_rfc3339(),
        ..Feed::default()
    };
    feed.entries = entry_list;
    feed
}
