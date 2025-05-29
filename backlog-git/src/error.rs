use thiserror::Error;

/// The common result type used throughout this crate for Git and Pull Request API operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that can occur when interacting with the Backlog Git and Pull Request API.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred in the underlying HTTP client (from `backlog-api-core`)
    /// during request processing (e.g., network issues, request building).
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] backlog_api_core::Error),

    /// An error reported by the Backlog API itself.
    /// This could be due to invalid parameters sent to the API, authentication failures,
    /// permission issues, or other API-specific problems.
    #[error("Backlog API error: {message}")]
    Api {
        /// A descriptive message from the API.
        message: String,
        // Optionally, include more details like error codes if the API provides them
        // errors: Vec<BacklogApiErrorDetail>, // Example
    },

    /// An error occurred during JSON serialization or deserialization.
    /// This typically indicates an issue with parsing the API response into expected Rust structs
    /// or serializing a request body.
    #[error("JSON processing error: {0}")]
    Json(#[from] serde_json::Error),

    /// A required resource (e.g., project, repository, pull request) was not found on the server.
    #[error("Resource not found: {resource_type} {identifier}")]
    NotFound {
        /// The type of resource that was not found (e.g., "repository", "pull request").
        resource_type: String,
        /// The identifier used to look up the resource.
        identifier: String,
    },

    /// An error indicating that an invalid argument was provided to an API function,
    /// distinct from API-level parameter errors.
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    // Add other specific error types as needed.
    // For example:
    // #[error("Pull request #{number} already merged")]
    // PullRequestAlreadyMerged { number: u64 },
}

// If the Backlog API returns structured error messages,
// you might define a struct for them like this:
//
// use serde::Deserialize;
// #[derive(Debug, Deserialize)]
// pub struct BacklogApiErrorDetail {
//     pub message: String,
//     pub code: Option<i32>,
//     pub more_info: Option<String>,
// }
//
// And then use it in the `Error::Api` variant.
// For now, keeping it simple with just a message string.
