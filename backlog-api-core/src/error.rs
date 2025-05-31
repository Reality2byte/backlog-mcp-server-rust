use thiserror::Error;
// Add use statement for backlog_core so its Error type can be referenced.
// use backlog_core; // This line is redundant as backlog_core::Error is used with its full path.

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Validation error: {0}")]
    Validation(#[from] backlog_core::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
