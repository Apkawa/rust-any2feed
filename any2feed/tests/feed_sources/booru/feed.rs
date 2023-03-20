use reqwest::Url;

use any2feed::feed_sources::booru::feed::{booru_posts_to_feed, Context};
use booru_rs::client::danbooru::DanbooruPost;
use booru_rs::client::generic::model::BooruPostModelSetUrl;
use booru_rs::client::generic::BooruPostModel;
use test_utils::fixture::load_json_fixture;

#[test]
fn test_entry_list_generic() {
    let p =
        serde_json::from_str::<Vec<DanbooruPost>>(load_json_fixture("danbooru/post_list").as_str())
            .unwrap()
            .set_base_url("https://booru.com");
    let p = p
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
        .collect();
    let _e = booru_posts_to_feed(p, None);
    // dbg!(e);
}

#[test]
fn test_entry_list_with_proxy() {
    let p =
        serde_json::from_str::<Vec<DanbooruPost>>(load_json_fixture("danbooru/post_list").as_str())
            .unwrap()
            .set_base_url("https://booru.com");
    let p = p
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn BooruPostModel>)
        .collect();
    let proxy_url = Some(Url::parse("http://localhost:123/proxy").unwrap());
    let _e = booru_posts_to_feed(
        p,
        Some(&Context {
            proxy_url,
            ..Context::default()
        }),
    );
    // dbg!(e);
}
