// https://validator.w3.org/feed/docs/atom.html

use chrono::Local;
use std::fmt::{Display, Formatter};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum Content {
    Text(String),
    Html(String),
    Xhtml(String),
}

impl Default for Content {
    fn default() -> Self {
        Content::Text(String::default())
    }
}

#[derive(Debug, Default)]
pub struct Element<T>(pub T);

#[derive(Debug, Default)]
pub struct CDATAElement<T>(pub T);

#[derive(Debug, Default)]
pub struct Attribute<T>(pub T);

#[derive(Debug, Default)]
pub struct Feed {
    pub id: String,
    pub title: CDATAElement<String>,
    pub updated: String,

    pub author: Element<Person>,
    pub link: Vec<Link>,

    pub categories: Option<Element<Vec<Category>>>,
    pub contributor: Option<Element<Person>>,
    pub generator: Option<Generator>,

    pub icon: Option<Element<String>>,
    pub logo: Option<Element<String>>,
    pub rights: Option<Content>,
    pub subtitle: Option<Content>,

    pub entries: Vec<Entry>,
}

#[derive(Debug, Default)]
pub struct Entry {
    pub id: String,
    pub title: CDATAElement<String>,
    // TODO dt object
    pub updated: String,
    pub author: Element<Person>,
    pub content: Option<Content>,
    pub link: Option<Link>,
    pub summary: Option<Content>,
    pub categories: Option<Element<Vec<Category>>>,
    pub contributor: Option<Element<Person>>,
    pub published: Option<Element<String>>,
    pub rights: Option<Content>,
}

impl Entry {
    pub fn new(id: String, title: String, updated: String) -> Entry {
        // TODO correct truncate unicode
        let title = {
            let title = if title.is_empty() {
                "no title".to_string()
            } else {
                title
            };
            if title.len() > 60 {
                UnicodeSegmentation::graphemes(title.as_str(), true)
                    .take(55)
                    .chain(["..."])
                    .collect::<String>()
            } else {
                title
            }
        };
        let id = {
            let mut id = id;
            if cfg!(debug_assertions) {
                log::warn!("Entry id DEBUG MODE");
                // Отладочный режим, делаем id неуникальными
                id.push('/');
                id.push_str(&Local::now().to_rfc3339());
            }
            id
        };

        Entry {
            id,
            title: CDATAElement(title),
            updated,
            ..Entry::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Category {
    pub term: String,
    pub scheme: Option<Attribute<String>>,
    pub label: Option<Attribute<String>>,
}

impl Category {
    pub fn new(term: String, scheme: Option<String>, label: Option<String>) -> Category {
        Category {
            term,
            scheme: scheme.map(Attribute),
            label: label.map(Attribute),
        }
    }
}

#[derive(Debug, Default)]
pub struct Person {
    pub name: String,
    pub url: Option<Element<String>>,
    pub email: Option<Element<String>>,
}

impl Person {
    pub fn new(name: String, url: Option<String>, email: Option<String>) -> Person {
        Person {
            name,
            url: url.map(Element),
            email: email.map(Element),
        }
    }
}

#[derive(Debug, Default)]
pub enum LinkRel {
    #[default]
    Alternate,
    Enclosure,
    Related,
    _Self,
    Via,
    /// https://www.rfc-editor.org/rfc/rfc5005#section-3
    First,
    Last,
    Previous,
    Next,
}

impl Display for LinkRel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{:?}", self)
                .trim_start_matches('_')
                .to_ascii_lowercase()
        )
    }
}

#[derive(Debug, Default)]
pub struct Link {
    pub href: Attribute<String>,
    pub title: Option<Attribute<String>>,
    pub length: Option<Attribute<usize>>,
    pub mime_type: Option<Attribute<String>>,
    pub rel: Option<Attribute<LinkRel>>,
    pub hreflang: Option<Attribute<String>>,
}

impl Link {
    pub fn new(href: String) -> Link {
        Link {
            href: Attribute(href),
            ..Link::default()
        }
    }
    pub fn with_rel(href: String, rel: LinkRel) -> Link {
        Link {
            href: Attribute(href),
            rel: Some(Attribute(rel)),
            ..Link::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Generator {
    pub name: Attribute<String>,
    pub uri: Option<Attribute<String>>,
    pub version: Option<Attribute<String>>,
}

impl Generator {
    pub fn new(name: String, uri: Option<String>, version: Option<String>) -> Generator {
        Generator {
            name: Attribute(name),
            uri: uri.map(Attribute),
            version: version.map(Attribute),
        }
    }
}
