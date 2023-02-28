use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

///
/// ```
/// use std::env;
/// use test_utils::fixture::get_git_root;
/// let r = get_git_root().unwrap();
/// assert!(r.ends_with("rust-any2feed"));
/// env::set_current_dir(r.join("test_utils/src/fixtures"));
/// let r = get_git_root().unwrap();
/// assert!(r.ends_with("rust-any2feed"));
/// ```
pub fn get_git_root() -> Option<PathBuf> {
    let mut dir = env::current_dir().ok()?;
    while dir.exists() {
        let p = Path::new(&dir).join(".git");
        if p.exists() && p.is_dir() {
            return Some(dir);
        }
        if !dir.pop() { break; }
    }
    None
}

///
/// ```
/// use test_utils::fixture::{load_fixture};
/// let str = load_fixture("allfeed.json");
/// assert!(str.len() > 0)
/// ```
pub fn load_fixture(name: &str) -> String {
    let fixture_path = get_git_root().unwrap()
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

