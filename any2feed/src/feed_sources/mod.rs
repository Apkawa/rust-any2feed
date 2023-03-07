use crate::feed_sources::mewe::feed_source::MeweFeedSource;
use crate::feed_sources::telegram::TelegramFeedSource;
use crate::feed_sources::traits::FeedSource;

pub mod mewe;
pub mod telegram;
pub mod traits;
pub mod utils;

pub struct FeedSourceList;

impl FeedSourceList {
    pub fn get_sources(toml: &str) -> Vec<Box<dyn FeedSource>> {
        vec![
            Box::new(MeweFeedSource::with_config(toml)),
            Box::new(TelegramFeedSource::with_config(toml)),
        ]
    }
}
