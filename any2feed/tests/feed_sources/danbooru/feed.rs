use any2feed::feed_sources::danbooru::feed::{danbooru_posts_to_feed, Context};
use reqwest::Url;
use test_utils::fixture::{load_fixture, load_json_fixture};

#[test]
fn test_entry_list_generic() {
    let p =
        serde_json::from_str(load_json_fixture(format!("danbooru/post_list").as_str()).as_str())
            .unwrap();
    let e = danbooru_posts_to_feed(p, None);
    dbg!(e);
}

#[test]
fn test_entry_list_with_proxy() {
    let p =
        serde_json::from_str(load_json_fixture(format!("danbooru/post_list").as_str()).as_str())
            .unwrap();
    let proxy_url = Some(Url::parse("http://localhost:123/proxy").unwrap());
    let e = danbooru_posts_to_feed(
        p,
        Some(&Context {
            proxy_url: proxy_url,
            ..Context::default()
        }),
    );
    dbg!(e);
}
