pub mod download_pull_request_attachment;
pub mod get_pull_request;
pub mod get_pull_request_attachment_list;
pub mod get_pull_request_comment_count;
pub mod get_pull_request_comment_list;
pub mod get_pull_request_count;
pub mod get_pull_request_list;
pub mod get_repository;
pub mod get_repository_list;

#[cfg(feature = "writable")]
pub mod add_pull_request;

#[cfg(feature = "writable")]
pub mod add_pull_request_comment;

#[cfg(feature = "writable")]
pub mod update_pull_request;

#[cfg(feature = "writable")]
pub mod update_pull_request_comment;

#[cfg(feature = "writable")]
pub mod delete_pull_request_attachment;
