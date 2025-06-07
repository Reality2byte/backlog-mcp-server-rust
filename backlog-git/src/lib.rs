//! `backlog-git`
//!
//! This crate provides functionalities to interact with the Git and Pull Request
//! features of the Backlog API.

pub mod api;
pub mod models;

// Re-export key types for easier access by users of this crate.
pub use api::GitApi;
pub use models::{IssueLink, PullRequest, PullRequestAttachment, PullRequestStatus, Repository};
