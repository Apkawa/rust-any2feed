use crate::feed_sources::mewe::feed_source::MeweFeedSource;
use crate::feed_sources::telegram::TelegramFeedSource;
use crate::feed_sources::traits::FeedSource;

pub mod error;
pub mod mewe;
pub mod telegram;
pub mod traits;
pub mod utils;

pub struct FeedSourceManager;

pub type FeedSourceList = Vec<Box<dyn FeedSource>>;

impl FeedSourceManager {
    pub fn get_sources() -> FeedSourceList {
        vec![
            Box::new(MeweFeedSource::default()),
            Box::new(TelegramFeedSource::default()),
        ]
    }
}
