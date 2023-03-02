use crate::traits::{FeedAttribute, FeedElement};
use crate::{Attribute, CDATAElement, Element};
use chrono::{DateTime, Local, Utc};
use std::fmt::{Display, Formatter};

/// http://opml.org/spec2.opml
///
//<?xml version='1.0' encoding='UTF-8' ?>
// <opml version="1.0">
//   <head>
//     <title>Thunderbird OPML Export - Blogs &amp; News Feeds</title>
//     <dateCreated>Wed, 22 Feb 2023 08:19:27 GMT</dateCreated>
//   </head>
//   <body>
// 		<outline text="Programming" title="Programming">
// 			<outline text="Better Programming - Medium" title="Better Programming - Medium" description="Advice for programmers. - Medium" xmlUrl="https://medium.com/feed/better-programming" type="rss" />
// 			<outline text="Code as Craft" title="Code as Craft" description="The Engineering Blog from Etsy" xmlUrl="https://codeascraft.com/feed/atom/" type="rss" />
//         </outline>
//   </body>
// </opml>
#[derive(Default, Debug)]
pub struct OPML {
    // version: Option<String>, todo select version
    pub title: CDATAElement<String>,
    pub outlines: Vec<Outline>,
    pub created: Option<DateTime<Utc>>,
}

impl OPML {
    pub fn new(title: &str) -> OPML {
        OPML {
            title: CDATAElement(title.to_string()),
            ..OPML::default()
        }
    }
    pub fn add_outline(mut self, outline: Outline) -> OPML {
        self.outlines.push(outline);
        self
    }
}

impl Display for OPML {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let OPML {
            title,
            created,
            outlines,
        } = self;

        let created = Element(created.unwrap_or_else(|| DateTime::from(Local::now())))
            .render_tag("dateCreated");
        let title = title.render_tag("title");
        let outlines = outlines.iter().map(|c| c.to_string()).collect::<String>();

        write!(
            f,
            r#"<?xml version="1.0" encoding="utf-8"?>
<opml version="1.0">
  <head>
    {title}
    {created}
  </head>
  <body>
        {outlines}
  </body>
</opml>
"#
        )
    }
}

#[derive(Default, Debug)]
pub struct Outline {
    pub title: Attribute<String>,
    pub description: Attribute<String>,
    pub text: Option<Attribute<String>>,
    pub xml_url: Option<Attribute<String>>,
    pub r#type: Option<Attribute<String>>,
    pub outlines: Option<Vec<Outline>>,
}

impl Outline {
    pub fn new(title: &str) -> Self {
        Self {
            title: Attribute(title.to_string()),
            description: Attribute(title.to_string()),
            ..Self::default()
        }
    }
    pub fn with_url(title: &str, url: &str) -> Self {
        Self {
            title: Attribute(title.to_string()),
            description: Attribute(title.to_string()),
            xml_url: Some(Attribute(url.to_string())),
            r#type: Some(Attribute(String::from("rss"))),
            ..Self::default()
        }
    }
    pub fn add_child(mut self, title: &str, url: Option<&str>) -> Outline {
        if self.outlines.is_none() {
            self.outlines = Some(Vec::with_capacity(3));
        }
        let outline = if let Some(url) = url {
            Outline::with_url(title, url)
        } else {
            Outline::new(title)
        };
        self.outlines.as_mut().unwrap().push(outline);
        self
    }

    pub fn add_outline(mut self, outline: Outline) -> Outline {
        if self.outlines.is_none() {
            self.outlines = Some(Vec::with_capacity(3));
        }
        self.outlines.as_mut().unwrap().push(outline);
        self
    }
}

impl Display for Outline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Outline {
            title,
            description,
            text,
            xml_url,
            r#type,
            outlines,
        } = self;
        let parts = [
            title.render_attr("title"),
            description.render_attr("description"),
            text.render_attr("text"),
            xml_url.render_attr("xmlUrl"),
            r#type.render_attr("type"),
        ]
        .join(" ");

        let outlines = outlines
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|c| c.to_string())
            .collect::<String>();
        let (start, end) = if !outlines.is_empty() {
            (">\n", "\n</outline>")
        } else {
            ("", " />")
        };
        write!(f, r#"<outline {parts}{start}{outlines}{end}"#)
    }
}

#[cfg(test)]
mod test {
    use crate::opml::{Outline, OPML};

    #[test]
    fn test_opml_render() {
        let mut opml = OPML::new("Foo bar");
        let mut outline = Outline::new("Foo");
        outline.outlines = Some(vec![
            Outline::with_url("bar & url", "http://ya.ru"),
            Outline::with_url("diez", "http://diez.ru"),
        ]);
        opml.outlines.push(outline);

        dbg!(&opml);
        println!("{}", opml);
    }

    #[test]
    fn test_opml_outline_fluent_pattern() {
        let outline3 = Outline::new("level 2").add_child("level 3", Some("https://example.com"));
        let opml = OPML::new("Foo bar").add_outline(
            Outline::new("Foo")
                .add_outline(outline3)
                .add_child("diez", Some("https://du.hast.much")),
        );
        dbg!(&opml);
        println!("{}", opml);
    }
}
