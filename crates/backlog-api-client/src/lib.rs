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
        PullRequestNumber, StatusId, UserId, WikiId, WikiTagId,
    },
};

// Document module (from backlog_document)
#[cfg(feature = "document")]
pub use backlog_document::{
    DocumentApi, DocumentDetail, DocumentTreeRootNode, DownloadAttachmentParams, GetDocumentParams,
    GetDocumentTreeParams, GetDocumentTreeResponse,
};

// File module (from backlog_file)
#[cfg(feature = "file")]
pub use backlog_file::models::FileContent;
#[cfg(feature = "file")]
pub use backlog_file::{FileApi, GetSharedFilesListParams, GetSharedFilesListResponse, SharedFile};

// Issue module (from backlog_issue)
#[cfg(feature = "issue")]
pub use backlog_issue::{
    Attachment, ChangeLogEntry, Comment, CommentOrder, GetCommentListParams,
    GetCommentListParamsBuilder, GetCommentNotificationsParams, GetIssueListParams,
    GetIssueListParamsBuilder, Issue, IssueApi, SharedFile as IssueSharedFile,
};

#[cfg(all(feature = "issue", feature = "issue_writable"))]
pub use backlog_issue::{
    AddCommentParams, AddCommentParamsBuilder, UpdateIssueParams, UpdateIssueParamsBuilder,
};

// Re-export backlog_issue module for parameter access
#[cfg(feature = "issue")]
pub use backlog_issue;

// Issue writable operations (from backlog_issue)
#[cfg(all(feature = "issue", feature = "issue_writable"))]
pub use backlog_issue::{LinkSharedFilesToIssueParams, LinkSharedFilesToIssueParamsBuilder};

// Git module (from backlog_git)
#[cfg(feature = "git")]
pub use backlog_git::{
    ChangeLog, DownloadPullRequestAttachmentParams, DownloadPullRequestAttachmentResponse,
    GetPullRequestAttachmentListParams, GetPullRequestAttachmentListResponse,
    GetPullRequestCommentCountParams, GetPullRequestCommentCountResponse,
    GetPullRequestCommentListParams, GetPullRequestCommentListResponse, GetPullRequestCountParams,
    GetPullRequestCountResponse, GetPullRequestListParams, GetPullRequestListResponse,
    GetPullRequestParams, GetPullRequestResponse, GetRepositoryListParams,
    GetRepositoryListResponse, GetRepositoryParams, GetRepositoryResponse, GitApi, Notification,
    PrCommentOrder, PullRequest, PullRequestAttachment, PullRequestComment,
    PullRequestCommentCount, PullRequestCount, Repository, Star,
};

#[cfg(all(feature = "git", feature = "git_writable"))]
pub use backlog_git::{
    AddPullRequestCommentParams, AddPullRequestCommentResponse, AddPullRequestParams,
    AddPullRequestResponse, DeletePullRequestAttachmentParams, DeletePullRequestAttachmentResponse,
    UpdatePullRequestCommentParams, UpdatePullRequestCommentResponse, UpdatePullRequestParams,
    UpdatePullRequestResponse,
};

// Project module (from backlog_project)
#[cfg(feature = "project")]
pub use backlog_project::{IssueType, Milestone, Project, ProjectApi, Status};

// Re-export backlog_project module for parameter access
#[cfg(feature = "project")]
pub use backlog_project;

// Space module (from backlog_space)
#[cfg(feature = "space")]
pub use backlog_space::{
    GetSpaceNotificationParams, GetSpaceNotificationResponse, SpaceApi, SpaceNotification,
};

// Space writable operations (from backlog_space)
#[cfg(all(feature = "space", feature = "space_writable"))]
pub use backlog_space::{UpdateSpaceNotificationParams, UploadAttachmentParams};

// User module (from backlog_user)
#[cfg(feature = "user")]
pub use backlog_user::UserApi;

// Activity module (from backlog_activity)
#[cfg(feature = "activity")]
pub use backlog_activity::{Activity, ActivityApi};

// Wiki module (from backlog_wiki)
#[cfg(feature = "wiki")]
pub use backlog_wiki::{
    DownloadWikiAttachmentParams, GetWikiAttachmentListParams, GetWikiAttachmentListResponse,
    GetWikiCountParams, GetWikiCountResponse, GetWikiDetailParams, GetWikiDetailResponse,
    GetWikiListParams, GetWikiListResponse, Wiki, WikiApi, WikiAttachment, WikiCount, WikiDetail,
    WikiTag,
};

#[cfg(all(feature = "wiki", feature = "wiki_writable"))]
pub use backlog_wiki::{UpdateWikiParams, UpdateWikiResponse};

// Team module (from backlog_team)
#[cfg(feature = "team")]
pub use backlog_team::{
    GetTeamParams, GetTeamResponse, ListTeamsOrder, ListTeamsParams, ListTeamsResponse, TeamApi,
};
