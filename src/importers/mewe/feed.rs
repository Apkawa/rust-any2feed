use std::collections::HashMap;
use std::vec;
use chrono::{Local};
use regex::Regex;
use reqwest::Url;
use crate::feed::{Category, CDATAElement, Content, Element, Entry, Feed, Link, Person};
use crate::importers::mewe::json::{
    MeweApiFeedList,
    MeweApiPost,
    MeweApiUserInfo,
};
use crate::importers::mewe::render_content::RenderContent;


pub fn mewe_post_to_entry(post: &MeweApiPost, author: Option<&MeweApiUserInfo>) -> Option<Entry> {
    let mut entry = Entry::new(
        post.id.to_string(),
        post.text.to_string(),
        // TODO dt
        post.updated_at.to_rfc3339(),
    );
    entry.published = Some(Element(post.created_at.to_rfc3339()));
    if let Some(hash_tags) = &post.hash_tags {
        let categories = hash_tags.iter()
            .map(|t| Category { term: t.to_string(), ..Category::default() })
            .collect::<Vec<Category>>();
        entry.categories = Some(Element(categories));
    }
    if let Some(content) = post.render() {
        entry.content = Some(Content::Html(content));
    }
    if let Some(author) = author {
        entry.author = Element(Person::new(author.name.clone(), None, None));
    }
    entry.link = post.get_post_url(author).map(Link::new);
    if let Some(hash_tags) = &post.hash_tags {
        let categories = hash_tags.iter()
            .map(|t| Category::new(t.clone(), None, None))
            .collect::<Vec<Category>>();
        entry.categories = Some(Element(categories));
    }
    Some(entry)
}

// Быстрофункция чтобы добить наконец функциональность до смотрибельного
pub fn mewe_feed_to_feed(feed_list: &Vec<MeweApiFeedList>) -> Option<Feed> {
    let mut entries: Vec<Entry> = Vec::with_capacity(feed_list.len() * 10);
    let mut authors: HashMap<&String, &MeweApiUserInfo> = HashMap::with_capacity(20);
    for list in feed_list.iter() {
        for user in list.users.iter() {
            authors.insert(&user.id, user);
        }
        for post in list.feed.iter() {
            let entry = mewe_post_to_entry(
                &post,
                Some(authors.get(&post.user_id).unwrap()))
                .expect(format!("{post:?}").as_str());
            entries.push(entry);
        }
    }
    let link = vec![Link::new("https://mewe.com/myworld".to_string())];


    let feed = Feed {
        id: "mewe_feed".to_string(),
        title: CDATAElement("Mewe feed".to_string()),
        link,
        updated: Local::now().to_rfc3339(),
        author: Element(Person { name: "Mewe".to_string(), ..Person::default() }),
        entries,
        ..Feed::default()
    };
    return Some(feed);
}

/// Меняем урлы на урл прокси
/// ```
/// use rust_any2feed::importers::mewe::feed::replace_mewe_media_urls;
/// let text = r#"<img src="https://mewe.com/api/v2/photo/c...0/200x300/img?static=0&mime=image/png" />
/// <video><source src="https://mewe.com/api/v2/proxy/video/shared/5...9/original/gplus7.mp4?_dummy=1"/>
/// </video>
/// "#;
/// let new_text: String = replace_mewe_media_urls(&text, &"http://127.0.0.1:12345/mewe/media");
/// let expect_text = r#"<img src="http://127.0.0.1:12345/mewe/media/api/v2/photo/c...0/200x300/img?static=0&mime=image/png" />
/// <video><source src="http://127.0.0.1:12345/mewe/media/api/v2/proxy/video/shared/5...9/original/gplus7.mp4?_dummy=1"/>
/// </video>
/// "#;
/// assert_eq!(new_text, expect_text);
/// ```
pub fn replace_mewe_media_urls(text: &str, new_url: &str) -> String {
    let re = Regex::new(r#"(?P<host>https://mewe.com)(?P<m>/api/v2/(?:photo|proxy/video)/)"#).unwrap();
    let res = re.replace_all(&text, &format!("{new_url}$m"));
    return res.to_string();
}

/// Из пути в прокси делаем прямой путь
/// ```
/// use rust_any2feed::importers::mewe::feed::get_media_url_from_proxy_path;
/// let path = "/mewe/media/api/v2/photo/c...0/200x300/img?static=0&mime=image/png";
/// let url = get_media_url_from_proxy_path(path).unwrap();
/// let expect = "https://mewe.com/api/v2/photo/c...0/200x300/img?static=0&mime=image/png";
/// assert_eq!(url.as_str(), expect);
/// ```
pub fn get_media_url_from_proxy_path(path: &str) -> Option<Url> {
    match path.split_once("v2") {
        Some((_, l)) =>
            Some(Url::parse(format!("https://mewe.com/api/v2{l}").as_str()).unwrap()),
        _ => None
    }
}
