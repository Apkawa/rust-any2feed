use rstest::rstest;
use rust_any2feed::importers::mewe::feed::{mewe_feed_to_feed, mewe_post_to_entry};

use rust_any2feed::importers::mewe::json::{MeweApiFeedList, MeweApiPost, MeweApiUserInfo};
use crate::importers::mewe;

#[test]
fn test_post_to_entry() {
    let json: MeweApiPost = serde_json::from_str(
        mewe::load_json_fixture("post_media").as_str()
    ).unwrap();
    let author = MeweApiUserInfo{
        name: "User name".to_string(),
        id: "5c25e3487d4f7e4447d07ba0".to_string(),
        ..MeweApiUserInfo::default()
    };
    let entry = mewe_post_to_entry(&json, Some(&author)).unwrap();
    dbg!(&entry);
}

#[rstest]
#[case("post_link")]
#[case("post_link_thumbnail")]
#[case("post_media")]
#[case("post_media_video")]
#[case("post_poll")]
#[case("post_text")]
fn test_post_to_entry_parametrized(#[case] json_name: &str) {
    let json: MeweApiPost = serde_json::from_str(mewe::load_json_fixture(json_name).as_str()).unwrap();
    let entry = mewe_post_to_entry(&json, None).unwrap();
    dbg!(&entry);
}

#[test]
fn test_allfeed() {
    let json: MeweApiFeedList = serde_json::from_str(
        mewe::load_json_fixture("allfeed").as_str()
    ).unwrap();
    let feeds = &vec![json];
    let entry = mewe_feed_to_feed(&feeds).unwrap();
    dbg!(&entry);
    let xml = entry.to_string();
    println!("{}", xml);

}
