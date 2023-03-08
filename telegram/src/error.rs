pub type Result<T> = std::result::Result<T, TelegramApiError>;

#[derive(Debug)]
pub enum TelegramApiErrorKind {
    IdentifyFail,
    StatusError,
}

#[derive(Debug)]
pub enum TelegramApiError {
    ApiError {
        kind: TelegramApiErrorKind,
    },
    ReqwestError(reqwest::Error),
    ParseError {
        error: String,
        context: Option<String>,
    },
}

impl From<reqwest::Error> for TelegramApiError {
    fn from(value: reqwest::Error) -> Self {
        TelegramApiError::ReqwestError(value)
    }
}

impl From<(scraper::error::SelectorErrorKind<'_>, String)> for TelegramApiError {
    fn from((value, context): (scraper::error::SelectorErrorKind, String)) -> Self {
        TelegramApiError::ParseError {
            error: format!("scraper::error::SelectorErrorKind::{:?}", value),
            context: Some(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TelegramApiError::ParseError;
    use super::*;

    #[test]
    fn test_error_context() {
        let error = scraper::error::SelectorErrorKind::EndOfLine;
        let error_with_context = (error, "error context".to_string());
        let tg_error = TelegramApiError::from(error_with_context);
        dbg!(&tg_error);
        if let ParseError {
            context: Some(context),
            error: _,
        } = tg_error
        {
            assert_eq!(context, "error context".to_string())
        }
    }
}
