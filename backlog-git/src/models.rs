//! Data models for Backlog Git and Pull Request entities.

use backlog_core::{
    User,
    identifier::{
        AttachmentId, IssueId, NotificationId, PrNumber, ProjectId, PullRequestCommentId,
        PullRequestId, RepositoryId, StarId,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize}; // Assuming User is defined in backlog-core and public, and implements Serialize, JsonSchema

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a Git repository in Backlog.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    /// The ID of the repository.
    pub id: RepositoryId,
    /// The ID of the project that the repository belongs to.
    pub project_id: ProjectId,
    /// The name of the repository.
    pub name: String,
    /// The description of the repository.
    pub description: Option<String>,
    /// The hook URL for the repository.
    pub hook_url: Option<String>, // Note: Nulab's API might use "nulabAccount" specific URLs
    /// The HTTP URL for cloning the repository.
    pub http_url: Option<String>,
    /// The SSH URL for cloning the repository.
    pub ssh_url: Option<String>,
    /// The display order of the repository.
    pub display_order: Option<u64>,
    /// The timestamp of the last push to the repository.
    pub pushed_at: Option<DateTime<Utc>>,
    /// The user who created the repository.
    pub created_user: Option<User>,
    /// The timestamp of when the repository was created.
    pub created: Option<DateTime<Utc>>,
    /// The user who last updated the repository.
    pub updated_user: Option<User>,
    /// The timestamp of when the repository was last updated.
    pub updated: Option<DateTime<Utc>>,
}

/// Represents a Pull Request in Backlog.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    /// The ID of the pull request.
    pub id: PullRequestId,
    /// The ID of the project that the pull request belongs to.
    pub project_id: ProjectId,
    /// The ID of the repository that the pull request belongs to.
    pub repository_id: RepositoryId,
    /// The number of the pull request, unique within the repository.
    pub number: PrNumber,
    /// The summary (title) of the pull request.
    pub summary: String,
    /// The description of the pull request.
    pub description: Option<String>,
    /// The name of the base branch (target branch).
    pub base: String,
    /// The name of the branch to be merged (source branch).
    pub branch: String,
    /// The status of the pull request.
    pub status: PullRequestStatus,
    /// The user assigned to the pull request.
    pub assignee: Option<User>,
    /// The issue related to this pull request, if any.
    #[serde(rename = "issue")]
    pub related_issue: Option<IssueLink>,
    /// The SHA hash of the base commit.
    pub base_commit: Option<String>,
    /// The SHA hash of the branch commit.
    pub branch_commit: Option<String>,
    /// The timestamp of when the pull request was closed.
    pub close_at: Option<DateTime<Utc>>,
    /// The timestamp of when the pull request was merged.
    pub merge_at: Option<DateTime<Utc>>,
    /// The user who created the pull request.
    pub created_user: Option<User>,
    /// The timestamp of when the pull request was created.
    pub created: Option<DateTime<Utc>>,
    /// The user who last updated the pull request.
    pub updated_user: Option<User>,
    /// The timestamp of when the pull request was last updated.
    pub updated: Option<DateTime<Utc>>,
    // attachments: Vec<Attachment>, // Define Attachment if needed
    // stars: Vec<Star>, // Define Star if needed
}

/// Represents the status of a Pull Request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestStatus {
    /// The ID of the pull request status.
    pub id: u64,
    /// The name of the pull request status (e.g., "Open", "Merged", "Closed").
    pub name: String,
}

/// Represents a simplified link to an issue, often used in Pull Request details.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IssueLink {
    /// The ID of the linked issue.
    pub id: IssueId,
    // Add more fields if the API provides them, like issueKey, summary
}

/// Represents an attachment associated with a pull request.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PullRequestAttachment {
    /// The unique identifier for the attachment.
    pub id: AttachmentId,
    /// The name of the attachment file.
    pub name: String,
    /// The size of the attachment file in bytes.
    pub size: u64,
}

/// Represents a star given to an entity in Backlog.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Star {
    pub id: StarId,
    pub comment: Option<String>,
    pub url: String,
    pub title: String,
    pub presenter: User,
    pub created: DateTime<Utc>,
}

/// Represents a notification related to an entity in Backlog.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Notification {
    pub id: NotificationId,
    pub already_read: bool,
    pub reason: u8,
    pub user: User,
    pub resource_already_read: bool,
}

/// Represents a log of a change made to a pull request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ChangeLog {
    pub field: String,
    pub new_value: String,
    pub original_value: Option<String>,
}

/// Represents a comment on a pull request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PullRequestComment {
    pub id: PullRequestCommentId,
    pub content: String,
    pub change_log: Vec<ChangeLog>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub stars: Vec<Star>,
    pub notifications: Vec<Notification>,
}
