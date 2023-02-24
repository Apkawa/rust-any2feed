use std::fmt::{Display, Formatter};
use std::format;
use crate::feed::{Attribute, CDATAElement, Element};
use crate::feed::utils::escape;

pub trait FeedElement {
    fn render_tag(&self, tag: &str) -> String;
}

impl<T: Display> FeedElement for Element<T> {
    fn render_tag(&self, tag: &str) -> String {
        format!("<{tag}>{}</{tag}>", self.0)
    }
}

impl<T: Display> FeedElement for CDATAElement<T> {
    fn render_tag(&self, tag: &str) -> String {
        format!("<{tag}><![CDATA[{}]]></{tag}>", self.0)
    }
}


impl<T: FeedElement> FeedElement for Option<T> {
    fn render_tag(&self, tag: &str) -> String {
        self.as_ref().map_or(String::new(), |e| e.render_tag(tag))
    }
}


impl<T: Display> Display for Element<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


pub trait FeedAttribute {
    fn render_attr(&self, name: &str) -> String;
}


impl<T: FeedAttribute> FeedAttribute for Option<T> {
    fn render_attr(&self, name: &str) -> String {
        self.as_ref().map_or(String::new(), |e| e.render_attr(name))
    }
}


impl<T: Display> FeedAttribute for Attribute<T> {
    fn render_attr(&self, name: &str) -> String {
        let s = format!(r#"{name}="{}""#, self.0);
        escape(s.as_str())
    }
}


