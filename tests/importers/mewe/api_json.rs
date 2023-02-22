use std::fs::read_to_string;
use rstest::rstest;

use rust_any2feed::importers::mewe::json::{MeweApiFeedList, MeweApiPost};

fn load_fixture(name: &str) -> String {
    read_to_string(
        format!("tests/importers/mewe/fixtures/{name}.json")).unwrap()
}

#[rstest]
#[case("post_link")]
#[case("post_link_thumbnail")]
#[case("post_media")]
#[case("post_media_video")]
#[case("post_poll")]
#[case("post_text")]
fn test_json(#[case] json_name: &str) {
    let json: MeweApiPost = serde_json::from_str(load_fixture(json_name).as_str()).unwrap();
    dbg!(&json);
}


#[test]
fn test_post_media() {
    let json: MeweApiPost = serde_json::from_str(load_fixture("post_media").as_str()).unwrap();
    dbg!(&json);
    assert_eq!(json.id, "63efdc34b8052935b96e0dd8");
    assert_eq!(json.hash_tags.unwrap(), vec!["weeklydose"]);
}

#[test]
fn test_allfeed() {
    let json: MeweApiFeedList = serde_json::from_str(load_fixture("allfeed").as_str()).unwrap();
    dbg!(&json);
}

#[test]
fn test_empty_feed() {
    let json: MeweApiFeedList = serde_json::from_str(load_fixture("allfeed_no_next_page").as_str()).unwrap();
    dbg!(&json);
}
