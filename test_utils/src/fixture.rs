use std::fs::read_to_string;
pub use test_helpers::fixture::{get_git_root, path_from_git_root};

///
/// ```
/// use test_utils::fixture::{load_fixture};
/// let str = load_fixture("allfeed.json");
/// assert!(str.len() > 0)
/// ```
pub fn load_fixture(name: &str) -> String {
    let fixture_path = get_git_root()
        .unwrap()
        .join(format!("test_utils/src/fixtures/{name}"));
    dbg!(&fixture_path);
    read_to_string(fixture_path).unwrap()
}

///
/// ```
/// use test_utils::fixture::load_json_fixture;
/// let str = load_json_fixture("allfeed");
/// assert!(str.len() > 0)
/// ```
pub fn load_json_fixture(name: &str) -> String {
    load_fixture(format!("{name}.json").as_str())
}
