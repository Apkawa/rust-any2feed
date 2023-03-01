use chrono::Local;
use feed::{Element, Entry, Feed, Link, Person, Content, CDATAElement};
use telegram::data::{Channel, ChannelPost};
use crate::importers::traits::RenderContent;

pub fn channel_post_to_entry(post: &ChannelPost) -> Entry {
    let title = post.text.lines().map(|l| l.trim()).collect::<Vec<_>>().join(" ");
    let mut id = post.id.clone();
    // Для отладки
    id.push_str(&Local::now().to_rfc3339());
    let mut entry = Entry::new(
        id,
        title,
        post.datetime.clone(),
    );

    entry.link = Some(Link::new(post.preview_url()));

    entry.content = Some(Content::Html(post.render().unwrap()));

    if let Some(from_author) = post.from_author.as_ref() {
        entry.author = Element(Person::new(from_author.clone(), None, None))
    }

    return entry;
}

pub fn channel_to_feed(channel: &Channel) -> Feed {
    let mut feed = Feed {
        title: CDATAElement(channel.title.clone()),
        updated: Local::now().to_rfc3339(),
        author: Element(Person::new(
            channel.title.clone(),
            Some(channel.preview_url()), None)),
        // todo logo
        subtitle: Some(Content::Text(channel.description.clone())),
        ..Feed::default()
    };
    feed.link.push(Link::new(channel.preview_url()));
    feed.entries = channel.posts.iter().map(channel_post_to_entry).collect();
    return feed;
}