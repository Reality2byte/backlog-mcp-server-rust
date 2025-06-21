pub mod client;
pub use ::client::DownloadedFile; // Re-export DownloadedFile from the client crate
pub use backlog_api_core::Error as ApiError;
pub use backlog_api_core::bytes; // Re-export bytes

// Core types (from backlog_core)
pub use backlog_core::{
    Error as CoreError,
    IssueIdOrKey,
    IssueKey,
    ProjectIdOrKey,
    RepositoryIdOrName,
    // User,
    // ProjectKey,
    // RepositoryName,
    // UserId,
    identifier::{
        AttachmentId, DocumentId, ProjectId, PullRequestAttachmentId, PullRequestCommentId,
        PullRequestNumber, StatusId, UserId,
    },
};

// Document module (from backlog_document)
#[cfg(feature = "document")]
pub use backlog_document::{
    DocumentApi, DocumentDetail, DocumentTreeResponse, DocumentTreeRootNode, GetDocumentTreeParams,
};

// File module (from backlog_file)
#[cfg(feature = "file")]
pub use backlog_file::{FileApi, GetSharedFilesListParams, GetSharedFilesListResponse, SharedFile};

// Issue module (from backlog_issue)
#[cfg(feature = "issue")]
pub use backlog_issue::{
    IssueApi,
    models::{Attachment, ChangeLogEntry, Comment, Issue, SharedFile as IssueSharedFile},
    requests::{
        AddCommentParams, AddCommentParamsBuilder, GetIssueListParams, GetIssueListParamsBuilder,
        UpdateIssueParams, UpdateIssueParamsBuilder,
        get_comment_list::{CommentOrder, GetCommentListParams, GetCommentListParamsBuilder},
    },
};

// Issue writable operations (from backlog_issue)
#[cfg(all(feature = "issue", feature = "issue_writable"))]
pub use backlog_issue::requests::{
    LinkSharedFilesToIssueParams, LinkSharedFilesToIssueParamsBuilder,
};

// Git module (from backlog_git)
#[cfg(feature = "git")]
pub use backlog_git::{
    api::GitApi,
    models::{
        ChangeLog, Notification, PrCommentOrder, PullRequest, PullRequestAttachment,
        PullRequestComment, PullRequestCommentCount, PullRequestCount, Repository, Star,
    },
    requests::{
        get_pull_request_comment_list::{
            GetPullRequestCommentListParams, GetPullRequestCommentListParamsBuilder,
        },
        get_pull_request_list::{GetPullRequestListParams, GetPullRequestListParamsBuilder},
    },
};

#[cfg(all(feature = "git", feature = "git_writable"))]
pub use backlog_git::requests::add_pull_request::{
    AddPullRequestParams, AddPullRequestParamsBuilder,
};

#[cfg(all(feature = "git", feature = "git_writable"))]
pub use backlog_git::requests::add_pull_request_comment::{
    AddPullRequestCommentParams, AddPullRequestCommentParamsBuilder,
};

#[cfg(all(feature = "git", feature = "git_writable"))]
pub use backlog_git::requests::update_pull_request::{
    UpdatePullRequestParams, UpdatePullRequestParamsBuilder,
};

#[cfg(all(feature = "git", feature = "git_writable"))]
pub use backlog_git::requests::update_pull_request_comment::{
    UpdatePullRequestCommentParams, UpdatePullRequestCommentParamsBuilder,
};

// Project module (from backlog_project)
#[cfg(feature = "project")]
pub use backlog_project::{IssueType, Milestone, Project, ProjectApi, Status};

// Space module (from backlog_space)
#[cfg(feature = "space")]
pub use backlog_space::SpaceApi;

// User module (from backlog_user)
#[cfg(feature = "user")]
pub use backlog_user::UserApi;
