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
    identifier::{ProjectId, StatusId, UserId},
};

// Document module (from backlog_document)
#[cfg(feature = "document")]
pub use backlog_document::{DocumentApi, DocumentDetail};

// Issue module (from backlog_issue)
#[cfg(feature = "issue")]
pub use backlog_issue::{
    Issue, IssueApi, Milestone,
    requests::{GetIssueListParamsBuilder, UpdateIssueParamsBuilder},
};

// Git module (from backlog_git)
#[cfg(feature = "git")]
pub use backlog_git::{GitApi, PullRequest, Repository};

// Project module (from backlog_project)
#[cfg(feature = "project")]
pub use backlog_project::{Project, ProjectApi};

// Space module (from backlog_space)
#[cfg(feature = "space")]
pub use backlog_space::SpaceApi;

// User module (from backlog_user)
#[cfg(feature = "user")]
pub use backlog_user::UserApi;
