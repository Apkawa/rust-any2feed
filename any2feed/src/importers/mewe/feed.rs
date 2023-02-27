use std::collections::HashMap;

use chrono::{NaiveDateTime, DateTime, Utc, Local, TimeZone};
use regex::Regex;
use reqwest::Url;

use feed::{Attribute, Category, CDATAElement, Content, Element, Entry, Feed, Link, Person};

use crate::importers::mewe::json::{MeweApiFeedList, MeweApiGroup, MeweApiPost, MeweApiUserInfo};
use crate::importers::mewe::render_content::RenderContent;

pub fn mewe_post_to_entry(post: &MeweApiPost,
                          author: Option<&MeweApiUserInfo>,
                          group: Option<&MeweApiGroup>) -> Option<Entry> {
    let post_url = post.get_post_url(author);
    let post_id = post_url.as_ref()
        .map_or(post.id.to_string(), |u| format!("{}/{}", u, post.id));
    let title = if post.text.is_empty() { "no title" } else { post.text.as_str() };
    let mut entry = Entry::new(
        post_id,
        title.to_string(),
        //
        post.edited_at.map(|e| Utc.timestamp_opt(e as i64, 0).unwrap()).unwrap_or(post.updated_at).to_rfc3339(),
    );
    entry.published = Some(Element(post.created_at.to_rfc3339()));
    let mut categories: Vec<Category> = Vec::with_capacity(2);
    if let Some(hash_tags) = &post.hash_tags {
        let it = hash_tags.iter()
            .map(|t| Category {
                term: format!("hashtag-{t}"),
                label: Some(Attribute(t.to_string())),
                ..Category::default()
            });
        categories.extend(it);
    }
    if let Some(group) = group {
        let name = group.name.clone();
        categories.push(Category {
            term: format!("{name}"),
            // label: Some(Attribute(name)),
            ..Category::default()
        })
    }
    if categories.len() > 0 {
        entry.categories = Some(Element(categories));
    }
    if let Some(content) = post.render() {
        entry.content = Some(Content::Html(content));
    }
    if let Some(author) = author {
        entry.author = Element(Person::new(author.name.clone(), None, None));
    }
    entry.link = post_url.map(Link::new);

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
    let mut groups: HashMap<&String, &MeweApiGroup> = HashMap::with_capacity(20);
    for list in feed_list.iter() {
        for user in list.users.iter() {
            authors.insert(&user.id, user);
        }
        if let Some(list_groups) = list.groups.as_ref() {
            for group in list_groups.iter() {
                groups.insert(&group.id, group);
            }
        }
        for post in list.feed.iter() {
            let author = authors.get(&post.user_id)
                .map(|a| *a);
            let group = post.group_id.as_ref()
                .map(|id| groups.get(&id))
                .flatten()
                .map(|o| *o);
            let entry = mewe_post_to_entry(
                post,
                author,
                group,
            )
                .unwrap_or_else(|| panic!("{post:?}"));
            entries.push(entry);
        }
    }

    let feed = Feed {
        id: "https://mewe.com/myworld".to_string(),
        title: CDATAElement("Mewe feed".to_string()),
        updated: Local::now().to_rfc3339(),
        author: Element(Person { name: "Mewe".to_string(), ..Person::default() }),
        entries,
        link: Vec::with_capacity(3),
        ..Feed::default()
    };
    Some(feed)
}

/// Меняем урлы на урл прокси
/// ```
/// use any2feed::importers::mewe::feed::replace_mewe_media_urls;
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
    let re = Regex::new(r#"(?P<host>https://mewe.com)(?P<m>/api/v2/(?:photo|proxy/video|doc/shared)/)"#).unwrap();
    let res = re.replace_all(text, &format!("{new_url}$m"));
    res.to_string()
}

/// Из пути в прокси делаем прямой путь
/// ```
/// use any2feed::importers::mewe::feed::get_media_url_from_proxy_path;
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
