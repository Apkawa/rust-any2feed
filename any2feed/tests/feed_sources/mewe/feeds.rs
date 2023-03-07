use any2feed::feed_sources::mewe::feed::{mewe_feed_to_feed, mewe_post_to_entry};
use rstest::rstest;

use mewe_api::json::{MeweApiFeedList, MeweApiPost, MeweApiUserInfo};
use test_utils::fixture;

#[test]
fn test_post_to_entry() {
    let mut json: MeweApiPost =
        serde_json::from_str(fixture::load_json_fixture("post_media").as_str()).unwrap();
    json.user = Some(MeweApiUserInfo {
        name: "User name".to_string(),
        id: "5c25e3487d4f7e4447d07ba0".to_string(),
        ..MeweApiUserInfo::default()
    });
    let entry = mewe_post_to_entry(&json).unwrap();
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
    let json: MeweApiPost =
        serde_json::from_str(fixture::load_json_fixture(json_name).as_str()).unwrap();
    let entry = mewe_post_to_entry(&json).unwrap();
    dbg!(&entry);
}

#[test]
fn test_allfeed() {
    let mut json: MeweApiFeedList =
        serde_json::from_str(fixture::load_json_fixture("allfeed").as_str()).unwrap();
    json.fill_user_and_group();
    let feeds = &vec![json];
    let entry = mewe_feed_to_feed(feeds).unwrap();
    dbg!(&entry);
    let xml = entry.to_string();
    println!("{}", xml);
}
