use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) danbooru: DanbooruFeedSourceConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DanbooruFeedSourceConfig {
    pub(crate) proxy: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) limit: Option<u32>,
}

impl DanbooruFeedSourceConfig {
    pub(crate) fn limit(&self) -> u32 {
        self.limit.unwrap_or(50)
    }
}
