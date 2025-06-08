pub mod client;
pub use ::client::DownloadedFile; // Re-export DownloadedFile from the client crate
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
    identifier::{AttachmentId, PrNumber, ProjectId, StatusId, UserId},
};

// Document module (from backlog_document)
#[cfg(feature = "document")]
pub use backlog_document::{
    DocumentApi, DocumentDetail, DocumentTreeResponse, DocumentTreeRootNode, GetDocumentTreeParams,
};

// Issue module (from backlog_issue)
#[cfg(feature = "issue")]
pub use backlog_issue::{
    Attachment, Issue, IssueApi,
    models::comment::{ChangeLogEntry, Comment, Notification, Star},
    requests::{
        GetIssueListParams, GetIssueListParamsBuilder, UpdateIssueParams, UpdateIssueParamsBuilder,
        get_comment_list::{CommentOrder, GetCommentListParams, GetCommentListParamsBuilder},
    },
};

// Git module (from backlog_git)
#[cfg(feature = "git")]
pub use backlog_git::{GitApi, PullRequest, PullRequestAttachment, Repository};

// Project module (from backlog_project)
#[cfg(feature = "project")]
pub use backlog_project::{IssueType, Milestone, Project, ProjectApi, Status};

// Space module (from backlog_space)
#[cfg(feature = "space")]
pub use backlog_space::SpaceApi;

// User module (from backlog_user)
#[cfg(feature = "user")]
pub use backlog_user::UserApi;
