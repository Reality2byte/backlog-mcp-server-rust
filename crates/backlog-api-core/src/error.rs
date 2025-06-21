use derive_builder::UninitializedFieldError;
use serde::Deserialize;
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

    #[error("Invalid build parameter error: {0}")]
    InvalidBuildParameter(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Backlog API Error (HTTP {status}): {errors_summary}")]
    HttpStatus {
        status: u16,
        errors: Vec<BacklogApiErrorEntry>,
        errors_summary: String, // Pre-formatted summary of errors
    },
    // Consider HttpErrorWithUnparsedBody { status: u16, body: String } later if needed
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<UninitializedFieldError> for Error {
    fn from(err: UninitializedFieldError) -> Self {
        Self::InvalidBuildParameter(err.to_string())
    }
}

/// Represents a single error entry from the Backlog API.
#[derive(Debug, Deserialize)]
pub struct BacklogApiErrorEntry {
    pub message: String,
    pub code: i64,
    #[serde(rename = "moreInfo")]
    pub more_info: Option<String>, // API can return empty string, map to Option
}

/// Represents the error response structure from the Backlog API.
#[derive(Debug, Deserialize)]
pub struct BacklogApiErrorResponse {
    pub errors: Vec<BacklogApiErrorEntry>,
}
