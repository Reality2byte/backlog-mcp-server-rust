pub mod client;
pub use backlog_api_core::Error as ApiError;
pub use backlog_api_core::bytes; // Re-export bytes

// Core types (from backlog_core)
pub use backlog_core::{
    DocumentId,
    Error as CoreError,
    IssueIdOrKey,
    IssueKey,
    ProjectIdOrKey,
    RepositoryIdOrName,
    // User,
    // ProjectKey,
    // RepositoryName,
    // UserId,
    identifier::{AttachmentId, ProjectId, StatusId, UserId}, // Added AttachmentId
};

// Document module (from backlog_document)
#[cfg(feature = "document")]
pub use backlog_document::{DocumentApi, DocumentDetail};

// Issue module (from backlog_issue)
#[cfg(feature = "issue")]
pub use backlog_issue::{
    Attachment, // Added Attachment
    Issue,
    IssueApi,
    Milestone,
    models::comment::{ChangeLogEntry, Comment, Notification, Star},
    requests::{
        GetIssueListParamsBuilder, UpdateIssueParamsBuilder,
        get_comment_list::{CommentOrder, GetCommentListParams, GetCommentListParamsBuilder},
    },
};

// Git module (from backlog_git)
#[cfg(feature = "git")]
pub use backlog_git::{GitApi, PullRequest, Repository};

// Project module (from backlog_project)
#[cfg(feature = "project")]
pub use backlog_project::{Project, ProjectApi, Status};

// Space module (from backlog_space)
#[cfg(feature = "space")]
pub use backlog_space::SpaceApi;

// User module (from backlog_user)
#[cfg(feature = "user")]
pub use backlog_user::UserApi;
