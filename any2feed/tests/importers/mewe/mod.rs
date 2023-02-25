use std::fs::read_to_string;

mod api_json;
mod feeds;

pub fn load_json_fixture(name: &str) -> String {
    read_to_string(
        format!("tests/importers/mewe/fixtures/{name}.json")).unwrap()
}
