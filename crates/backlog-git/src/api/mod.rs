mod download_pull_request_attachment;
mod get_pull_request;
mod get_pull_request_attachment_list;
mod get_pull_request_comment_count;
mod get_pull_request_comment_list;
mod get_pull_request_count;
mod get_pull_request_list;
mod get_repository;
mod get_repository_list;
mod git_api;

#[cfg(feature = "writable")]
mod add_pull_request;
#[cfg(feature = "writable")]
mod add_pull_request_comment;
#[cfg(feature = "writable")]
mod delete_pull_request_attachment;
#[cfg(feature = "writable")]
mod update_pull_request;
#[cfg(feature = "writable")]
mod update_pull_request_comment;

// Export response types (always available)
pub use download_pull_request_attachment::{
    DownloadPullRequestAttachmentParams, DownloadPullRequestAttachmentResponse,
};
pub use get_pull_request::{GetPullRequestParams, GetPullRequestResponse};
pub use get_pull_request_attachment_list::{
    GetPullRequestAttachmentListParams, GetPullRequestAttachmentListResponse,
};
pub use get_pull_request_comment_count::{
    GetPullRequestCommentCountParams, GetPullRequestCommentCountResponse,
};
pub use get_pull_request_comment_list::{
    GetPullRequestCommentListParams, GetPullRequestCommentListResponse,
};
pub use get_pull_request_count::{GetPullRequestCountParams, GetPullRequestCountResponse};
pub use get_pull_request_list::{GetPullRequestListParams, GetPullRequestListResponse};
pub use get_repository::{GetRepositoryParams, GetRepositoryResponse};
pub use get_repository_list::{GetRepositoryListParams, GetRepositoryListResponse};

// Export writable types with feature gates
#[cfg(feature = "writable")]
pub use add_pull_request::{AddPullRequestParams, AddPullRequestResponse};
#[cfg(feature = "writable")]
pub use add_pull_request_comment::{AddPullRequestCommentParams, AddPullRequestCommentResponse};
#[cfg(feature = "writable")]
pub use delete_pull_request_attachment::{
    DeletePullRequestAttachmentParams, DeletePullRequestAttachmentResponse,
};
#[cfg(feature = "writable")]
pub use update_pull_request::{UpdatePullRequestParams, UpdatePullRequestResponse};
#[cfg(feature = "writable")]
pub use update_pull_request_comment::{
    UpdatePullRequestCommentParams, UpdatePullRequestCommentResponse,
};

pub use git_api::GitApi;
