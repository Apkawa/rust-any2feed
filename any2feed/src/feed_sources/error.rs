use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum FeedSourceErrorKind {
    ApiError,
    ConfigError,
}

#[derive(Debug)]
pub struct FeedSourceError {
    pub kind: FeedSourceErrorKind,
    pub msg: String,
    pub detail: String,
}

impl From<TomlError> for FeedSourceError {
    fn from(value: TomlError) -> Self {
        FeedSourceError {
            kind: FeedSourceErrorKind::ConfigError,
            msg: "Toml de error".to_string(),
            detail: format!("{value:?}"),
        }
    }
}
