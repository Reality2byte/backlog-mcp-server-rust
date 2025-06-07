//! Data models for Backlog Git and Pull Request entities.

use backlog_core::{
    User,
    identifier::{AttachmentId, IssueId, ProjectId, PullRequestId, RepositoryId},
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
    #[serde(rename = "projectId")]
    pub project_id: u64,
    /// The name of the repository.
    pub name: String,
    /// The description of the repository.
    pub description: Option<String>,
    /// The hook URL for the repository.
    #[serde(rename = "hookUrl")]
    pub hook_url: Option<String>, // Note: Nulab's API might use "nulabAccount" specific URLs
    /// The HTTP URL for cloning the repository.
    #[serde(rename = "httpUrl")]
    pub http_url: Option<String>,
    /// The SSH URL for cloning the repository.
    #[serde(rename = "sshUrl")]
    pub ssh_url: Option<String>,
    /// The display order of the repository.
    #[serde(rename = "displayOrder")]
    pub display_order: Option<u64>,
    /// The timestamp of the last push to the repository.
    #[serde(rename = "pushedAt")]
    pub pushed_at: Option<DateTime<Utc>>,
    /// The user who created the repository.
    #[serde(rename = "createdUser")]
    pub created_user: Option<User>,
    /// The timestamp of when the repository was created.
    pub created: Option<DateTime<Utc>>,
    /// The user who last updated the repository.
    #[serde(rename = "updatedUser")]
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
    #[serde(rename = "projectId")]
    pub project_id: ProjectId,
    /// The ID of the repository that the pull request belongs to.
    #[serde(rename = "repositoryId")]
    pub repository_id: RepositoryId,
    /// The number of the pull request, unique within the repository.
    pub number: u64,
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
    #[serde(rename = "baseCommit")]
    pub base_commit: Option<String>,
    /// The SHA hash of the branch commit.
    #[serde(rename = "branchCommit")]
    pub branch_commit: Option<String>,
    /// The timestamp of when the pull request was closed.
    #[serde(rename = "closeAt")]
    pub close_at: Option<DateTime<Utc>>,
    /// The timestamp of when the pull request was merged.
    #[serde(rename = "mergeAt")]
    pub merge_at: Option<DateTime<Utc>>,
    /// The user who created the pull request.
    #[serde(rename = "createdUser")]
    pub created_user: Option<User>,
    /// The timestamp of when the pull request was created.
    pub created: Option<DateTime<Utc>>,
    /// The user who last updated the pull request.
    #[serde(rename = "updatedUser")]
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

// TODO: Define other related models like PullRequestComment, Attachment, Star if needed.
