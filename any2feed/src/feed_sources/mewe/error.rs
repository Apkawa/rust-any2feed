use crate::feed_sources::error::{FeedSourceError, FeedSourceErrorKind};
use mewe_api::MeweApiError;

impl From<MeweApiError> for FeedSourceError {
    fn from(value: MeweApiError) -> Self {
        FeedSourceError {
            kind: FeedSourceErrorKind::ApiError,
            msg: "Mewe api error".to_string(),
            detail: format!("{value:?}"),
        }
    }
}
