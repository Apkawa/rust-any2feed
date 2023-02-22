use rstest::rstest;
use rust_any2feed::importers::mewe::feed::mewe_post_to_entry;

use rust_any2feed::importers::mewe::json::{MeweApiFeedList, MeweApiPost};
use crate::importers::mewe;

#[test]
fn test_post_to_entry() {
    let json: MeweApiPost = serde_json::from_str(
        mewe::load_json_fixture("post_media").as_str()
    ).unwrap();
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
    let json: MeweApiPost = serde_json::from_str(mewe::load_json_fixture(json_name).as_str()).unwrap();
    let entry = mewe_post_to_entry(&json).unwrap();
    dbg!(&entry);
}
