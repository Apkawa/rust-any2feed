use std::fmt::{Display, Formatter};
use crate::feed::data::{
    Person,
    Content,
    Entry,
    Feed,
    Category,
};
use crate::feed::Link;
use crate::feed::traits::{FeedAttribute, FeedElement};
use crate::feed::utils::escape;


impl Display for Feed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Feed {
            id,
            title,
            link,
            updated,
            author,
            generator: _, // TODO
            subtitle,
            contributor,
            categories,
            icon,
            logo,
            rights,
            entries,
        } = self;
        let parts = [
            title.render_tag("title"),
            author.render_tag("author"),
            link.iter().map(|l| l.to_string()).collect::<String>(),
            // generator.render_
            subtitle.render_tag("subtitle"),
            contributor.render_tag("contributor"),
            icon.render_tag("icon"),
            logo.render_tag("logo"),
            rights.render_tag("rights"),
            categories.as_ref()
                .map_or(String::new(),
                        |l|
                            l.0.iter()
                                .map(|c| c.to_string()).collect(),
                ),
            entries.iter().map(|e| format!("{e}")).collect::<String>(),
        ].into_iter().filter(|s| s.len() > 0).collect::<Vec<String>>().join("\n");

        write!(f, r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <id>{id}</id>
  <updated>{updated}</updated>
  {parts}
</feed>"#
        )
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Entry {
            id,
            title,
            updated,
            author,
            content,
            link,
            summary,
            categories,
            contributor,
            published,
            rights
        } = self;

        let parts = [
            title.render_tag("title"),
            author.render_tag("author"),
            content.render_tag("content"),
            link.as_ref().map_or(String::new(),
                                 |l| l.to_string(),
            ),
            summary.render_tag("summary"),
            categories.as_ref()
                .map_or(String::new(),
                        |l|
                            l.0.iter()
                                .map(|c| c.to_string()).collect(),
                ),
            contributor.render_tag("contributor"),
            published.render_tag("published"),
            rights.render_tag("rights"),
        ].join("");
        write!(f, r#"
    <entry>
        <id>{id}</id>
        <updated>{updated}</updated>
        {parts}
    </entry>
        "#)
    }
}


impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Category {
            term,
            scheme,
            label
        } = self;
        let term = escape(term);
        let scheme = scheme.render_attr("scheme");
        let label = label.render_attr("label");
        write!(f, r#"<category term="{term}" {scheme} {label} />"#)
    }
}


impl FeedElement for Content {
    fn render_tag(&self, tag: &str) -> String {
        let (t, c) = match self {
            Content::Text(x) => { ("text", x) }
            Content::Html(x) => { ("html", x) }
            Content::Xhtml(x) => { ("xhtml", x) }
        };
        format!(r#"<{tag} type="{t}"><![CDATA[{c}]]></{tag}>"#)
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Link {
            href,
            title,
            length,
            mime_type,
            rel,
            hreflang
        } = self;
        let parts = [
            href.render_attr("href"),
            title.render_attr("title"),
            length.render_attr("length"),
            mime_type.render_attr("type"),
            rel.render_attr("rel"),
            hreflang.render_attr("hreflang"),
        ].join(" ");
        write!(f, r#"<link {parts}/>"#)
    }
}



impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Person {
            name,
            url,
            email,
        } = self;
        let url = url.render_tag("url");
        let email = email.render_tag("email");
        write!(f, r#"<name>{name}</name>{url}{email}"#)
    }
}

