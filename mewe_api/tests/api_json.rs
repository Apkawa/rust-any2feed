use rstest::rstest;

use mewe_api::json::{MeweApiContactList, MeweApiFeedList, MeweApiGroupList, MeweApiPost};
use test_utils::fixture;

#[rstest]
#[case("post_link")]
#[case("post_link_thumbnail")]
#[case("post_media")]
#[case("post_media_video")]
#[case("post_poll")]
#[case("post_ref_post")]
#[case("post_text")]
#[case("post_file")]
#[case("post_sticker")]
fn test_json(#[case] json_name: &str) {
    let json: MeweApiPost =
        serde_json::from_str(fixture::load_json_fixture(json_name).as_str()).unwrap();
    dbg!(&json);
}

#[test]
fn test_post_media() {
    let json: MeweApiPost =
        serde_json::from_str(fixture::load_json_fixture("post_media").as_str()).unwrap();
    dbg!(&json);
    assert_eq!(json.id, "63efdc34b8052935b96e0dd8");
    assert_eq!(json.hash_tags.unwrap(), vec!["weeklydose"]);
}

#[test]
fn test_allfeed() {
    let json: MeweApiFeedList =
        serde_json::from_str(fixture::load_json_fixture("allfeed").as_str()).unwrap();
    dbg!(&json);
}

#[test]
fn test_fill_user_group() {
    let mut json: MeweApiFeedList =
        serde_json::from_str(fixture::load_json_fixture("allfeed").as_str()).unwrap();
    json.fill_user_and_group();

    assert_eq!(json.feed[0].user_id, json.feed[0].user.as_ref().unwrap().id);
    dbg!(&json);
}

#[test]
fn test_empty_feed() {
    let json: MeweApiFeedList =
        serde_json::from_str(fixture::load_json_fixture("allfeed_no_next_page").as_str()).unwrap();
    dbg!(&json);
}

#[test]
fn test_group_list() {
    let json: MeweApiGroupList =
        serde_json::from_str(fixture::load_json_fixture("groups").as_str()).unwrap();
    dbg!(&json);
}

#[test]
fn test_contacts() {
    let json: MeweApiContactList =
        serde_json::from_str(fixture::load_json_fixture("contacts").as_str()).unwrap();
    dbg!(&json);
}
