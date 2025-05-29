//! `backlog-git`
//!
//! This crate provides functionalities to interact with the Git and Pull Request
//! features of the Backlog API.

pub mod error;
pub mod handler; // Changed from api to handler
pub mod models;

// Re-export key types for easier access by users of this crate.
pub use error::{Error, Result};
pub use handler::GitHandler;
pub use models::{IssueLink, PullRequest, PullRequestStatus, Repository}; // Re-export GitHandler instead of individual functions
