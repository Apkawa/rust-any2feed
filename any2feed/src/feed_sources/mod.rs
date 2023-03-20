use crate::feed_sources::booru::BooruFeedSource;
use crate::feed_sources::mewe::feed_source::MeweFeedSource;
use crate::feed_sources::telegram::TelegramFeedSource;
use crate::feed_sources::traits::FeedSource;

pub mod error;
pub mod traits;
pub mod utils;

// Feed sources
pub mod booru;
pub mod mewe;
pub mod telegram;

pub struct FeedSourceManager;

pub type FeedSourceList = Vec<Box<dyn FeedSource>>;

impl FeedSourceManager {
    pub fn get_sources() -> FeedSourceList {
        vec![
            Box::new(MeweFeedSource::default()),
            Box::new(TelegramFeedSource::default()),
            Box::new(BooruFeedSource::default()),
        ]
    }

    pub fn source_names() -> Vec<String> {
        FeedSourceManager::get_sources()
            .into_iter()
            .map(|s| s.name())
            .collect()
    }
}
