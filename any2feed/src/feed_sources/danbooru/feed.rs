use ::feed::Entry;
use booru_rs::model::danbooru::DanbooruPost;
use chrono::Local;
use feed::{Category, Content, Element, Feed, Link, Person};
use reqwest::Url;

#[derive(Debug, Default)]
pub struct Context {
    pub proxy_url: Option<Url>,
    pub key: Option<String>,
}

pub fn build_proxy_url(media_url: &str, mut proxy_url: Url) -> String {
    proxy_url.query_pairs_mut().append_pair("url", media_url);
    proxy_url.to_string()
}

pub fn danbooru_post_to_entry(post: DanbooruPost, context: Option<&Context>) -> Entry {
    let source = if post.pixiv_id.is_some() {
        format!("https://www.pixiv.net/artworks/{}", post.pixiv_id.unwrap())
    } else {
        post.source.clone()
    };
    let img = if post.file_url.is_none() {
        String::new()
    } else {
        // let img = DanbooruImage::from_md5(&post.md5.unwrap(), &post.file_ext);
        let sample = post.large_file_url.unwrap();
        let sample = if let Some(Context {
            proxy_url: Some(proxy_url),
            ..
        }) = context
        {
            build_proxy_url(&sample, proxy_url.clone())
        } else {
            sample
        };

        format!(
            r#"
        <img
          src="{sample}"
          alt="{alt}" />
        "#,
            sample = sample,
            alt = post.tag_string,
        )
    };
    let content = format!(
        r#"
        <a href="{source}">{source}</a>
        {img}
        "#,
    );
    let mut post_id = post.id.to_string();
    if let Some(Context { key: Some(key), .. }) = context {
        // For intersection feeds
        post_id.push_str(key)
    }
    let mut entry = Entry::new(post_id, post.tag_string, post.created_at.clone());
    entry.published = Some(Element(post.created_at));
    entry.content = Some(Content::Html(content));
    entry.link = Some(Link::new(format!(
        "https://danbooru.donmai.us/posts/{}",
        post.id
    )));
    let categories: Vec<Category> = post
        .tag_string_general
        .split_whitespace()
        .map(|t| Category::new(t.to_string(), None, None))
        .collect();
    entry.categories = Some(Element(categories));
    entry.author = Element(Person::new(post.tag_string_artist, None, None));
    entry
}

pub fn danbooru_posts_to_feed(posts: Vec<DanbooruPost>, context: Option<&Context>) -> Feed {
    let entry_list: Vec<Entry> = posts
        .into_iter()
        .map(|p| danbooru_post_to_entry(p, context))
        .collect();

    let mut feed = Feed {
        updated: Local::now().to_rfc3339(),
        author: Element(Person::new("Danbooru".to_string(), None, None)),
        ..Feed::default()
    };
    feed.entries = entry_list;
    feed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {}
}
