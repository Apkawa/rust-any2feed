// https://validator.w3.org/feed/docs/atom.html

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
    pub link: Link,

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
    pub title: String,
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
    pub fn new(id: String,
               title: String,
               updated: String,
    ) -> Entry {
        // TODO correct truncate unicode
        let title = {
            if title.len() > 60 {
                UnicodeSegmentation::graphemes(title.as_str(), true)
                    .take(55)
                    .chain(["..."])
                    .collect::<String>()
            } else {
                title
            }

        };
        Entry {
            id,
            title,
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
            scheme: scheme.map_or(None, |s| Some(Attribute(s))),
            label: label.map_or(None, |s| Some(Attribute(s))),
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
            url: url.map_or(None, |s| Some(Element(s))),
            email: email.map_or(None, |s| Some(Element(s))),
        }
    }
}

#[derive(Debug)]
pub enum LinkRel {
    Alternate,
    Enclosure,
    Related,
    _Self,
    Via,
}

impl Default for LinkRel {
    fn default() -> Self {
        LinkRel::Alternate
    }
}

impl Display for LinkRel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).trim_start_matches("_").to_ascii_lowercase())
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
        Link { href: Attribute(href), ..Link::default() }
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
            uri: uri.map_or(None, |s| Some(Attribute(s))),
            version: version.map_or(None, |s| Some(Attribute(s))),
        }
    }
}
