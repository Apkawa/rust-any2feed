use feed::{CDATAElement, Content, Element, Entry, Feed, Link, Person};

use feed::traits::FeedElement;

#[test]
#[ignore] // Todo
fn example_feed() {
    let f = Feed {
        title: CDATAElement("Foo".to_string()),
        link: vec![Link::new("https://foo.exe".to_string())],
        subtitle: Some(Content::Text("Foo".to_string())),
        updated: "2001-07-08T00:34:60".to_string(),
        icon: Some(Element("icon".to_string())),
        logo: Some(Element("icon".to_string())),
        entries: vec![
            Entry::new(
                "id".to_string(),
                "title".to_string(),
                "2001-07-08T00:34:60".to_string(),
            )
        ],
        ..Feed::default()
    };
    let s = f.to_string();
    assert_eq!(r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">

  <link href="https://foo.exe"     />
  <updated>2001-07-08T00:34:60</updated>
  <title><![CDATA[Foo]]</title>
<author><name></name></author>
<subtitle type="text><![CDATA[Foo]]</subtitle>
<icon>icon</icon>
<logo>icon</logo>

    <entry>
        <id>id</id>
        <title>title</title>
        <updated>2001-07-08T00:34:60</updated>
        <author><name></name></author>
    </entry>

</feed>"#, s);
}

#[test]
fn option_element() {
    let e = Some(Element(String::new()));
    let e2: Option<Element<String>> = None;
    let f = Some(Element("fill".to_string()));
    assert_eq!(e.render_tag("empty"), "<empty></empty>");
    assert_eq!(e2.render_tag("empty"), "");
    assert_eq!(f.render_tag("f"), "<f>fill</f>");
}

#[test]
fn person() {
    let p = Person {
        name: "Foo".to_string(),
        url: Some(Element("https://example.com".to_string())),
        email: None,
    };
    assert_eq!(p.to_string(), "<name>Foo</name><url>https://example.com</url>");
}


fn entry() {
    let _entry = Entry::new(
        "id".to_string(),
        "title".to_string(),
        "2001-07-08T00:34:60".to_string(),
    );
}

#[test]
fn entry_with_looong_text() {
    let entry = Entry::new(
        "id".to_string(),
        r#"Задремала тут днём, и приснилось что меве это такая гостинница на краю мира, наполненная деревянными автоматонами, которые помогают за ней присматривать. Из человеческого персонала там девушка которая старательно пытается быть монашкой, непонятный угрюмый подросток, который единственный понимает как эти автоматоны работают, и пара тётушек, которые ходят с лицом будто пережили войну."#.to_string(),
        "2001-07-08T00:34:60".to_string(),
    );
    assert_eq!(entry.title.0, "Задремала тут днём, и приснилось что меве это такая гос...");
}
