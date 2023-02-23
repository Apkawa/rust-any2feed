use std::collections::HashMap;
use crate::feed::{Category, CDATAElement, Content, Element, Entry, Feed, Person};
use crate::importers::mewe::json::{
    MeweApiFeedList,
    MeweApiPost,
    MeweApiUserInfo,
};
use crate::importers::mewe::render_content::RenderContent;


pub fn mewe_post_to_entry(post: &MeweApiPost) -> Option<Entry> {
    let mut entry = Entry::new(
        post.id.to_string(),
        post.text.to_string(),
        // TODO dt
        "2023-03-21T12:00:00".to_string(),
    );
    if let Some(hash_tags) = &post.hash_tags {
        let categories = hash_tags.iter()
            .map(|t| Category { term: t.to_string(), ..Category::default() })
            .collect::<Vec<Category>>();
        entry.categories = Some(Element(categories));
    }
    if let Some(content) = post.render() {
        entry.content = Some(Content::Html(content));
    }
    Some(entry)
}

// Быстрофункция чтобы добить наконец функциональность до смотрибельного
pub fn mewe_feed_to_feed(feed_list: &Vec<MeweApiFeedList>) -> Option<Feed> {
    let mut entries: Vec<Entry> = Vec::with_capacity(feed_list.len() * 10);
    let mut authors: HashMap<&String, &MeweApiUserInfo> = HashMap::with_capacity(20);
    for list in feed_list.iter() {
        for user in list.users.iter() {
            authors.insert(&user.id, &user);
        }
        for post in list.feed.iter() {
            let entry = mewe_post_to_entry(&post)
                .expect(format!("{post:?}").as_str());
            entries.push(entry);
        }
    }

    let feed = Feed {
        id: "mewe_feed".to_string(),
        title: CDATAElement("Mewe feed".to_string()),
        // TODO dt
        updated: "2023-03-21T12:00:00".to_string(),
        author: Element(Person { name: "Mewe".to_string(), ..Person::default() }),
        entries,
        ..Feed::default()
    };
    return Some(feed);
}