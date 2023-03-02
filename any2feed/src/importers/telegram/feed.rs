use crate::importers::traits::RenderContent;
use chrono::Local;
use feed::{CDATAElement, Content, Element, Entry, Feed, Link, Person};
use telegram::data::{Channel, ChannelPost};

pub fn channel_post_to_entry(post: &ChannelPost) -> Entry {
    let title = post
        .text
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join(" ");

    let mut entry = Entry::new(post.id.clone(), title, post.datetime.clone());

    entry.link = Some(Link::new(post.preview_url()));

    entry.content = Some(Content::Html(post.render().unwrap()));

    // TODO автора поста в канале лучше отображать где нибудь незаметно в тексте
    // entry.author = Element(Person::new(from_author.clone(), None, None))

    return entry;
}

pub fn channel_to_feed(channel: &Channel) -> Feed {
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
            let mut e = channel_post_to_entry(p);
            e.author = Element(Person::new(
                channel.title.clone(),
                Some(channel.preview_url()),
                None,
            ));
            e
        })
        .collect();
    return feed;
}
