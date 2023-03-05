use any2feed::importers::telegram::feed::channel_post_to_entry;
use telegram::data::ChannelPost;
use telegram::parse::parse_message;
use test_utils::fixture::load_fixture;

fn load_channel_post_fixture(name: &str) -> ChannelPost {
    let html = load_fixture(format!("telegram_preview/message_{name}.html").as_str());
    parse_message(html.as_str()).unwrap()
}

#[test]
fn test_entry_text() {
    let p = load_channel_post_fixture("text");
    let e = channel_post_to_entry(p, None);
    dbg!(e);
}
