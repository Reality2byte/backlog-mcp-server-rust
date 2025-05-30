pub mod client;
pub use backlog_api_core::Error as ApiError;

// Core types (from backlog_core)
pub use backlog_core::{
    DocumentId,
    Error as BacklogCoreError,
    IssueIdOrKey,
    IssueKey,
    ProjectIdOrKey,
    RepositoryIdOrName,
    // User,
    // ProjectKey,
    // RepositoryName,
    // UserId,
};

// Document module (from backlog_document)
#[cfg(feature = "document")]
pub use backlog_document::DocumentApi;
#[cfg(feature = "document")]
pub use backlog_document::models::DocumentDetail;

// Issue module (from backlog_issue)
#[cfg(feature = "issue")]
pub use backlog_issue::IssueApi;
#[cfg(feature = "issue")]
pub use backlog_issue::requests::{GetIssueListParamsBuilder, UpdateIssueParamsBuilder};
#[cfg(feature = "issue")]
pub use backlog_issue::{Issue, Milestone};

// Git module (from backlog_git)
#[cfg(feature = "git")]
pub use backlog_git::GitHandler;
#[cfg(feature = "git")]
pub use backlog_git::models::{PullRequest, Repository};

// Project module (from backlog_project)
#[cfg(feature = "project")]
pub use backlog_project::ProjectApi;
#[cfg(feature = "project")]
pub use backlog_project::models::Project;

// Space module (from backlog_space)
#[cfg(feature = "space")]
pub use backlog_space::SpaceApi;

// User module (from backlog_user)
#[cfg(feature = "user")]
pub use backlog_user::UserApi;
