use backlog_core::{
    User,
    identifier::{ProjectId, SharedFileId},
};
use chrono::{DateTime, Utc};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the content type of a shared file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileContent {
    /// A regular file with size information
    File { size: u64 },
    /// A directory (no size information)
    Directory,
}

/// Represents a shared file in Backlog.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SharedFile {
    /// The ID of the shared file.
    pub id: SharedFileId,
    /// The project ID of the shared file.
    pub project_id: ProjectId,
    /// The directory path of the shared file.
    pub dir: String,
    /// The name of the shared file.
    pub name: String,
    /// The user who created the shared file.
    pub created_user: User,
    /// The timestamp of when the shared file was created.
    pub created: DateTime<Utc>,
    /// The user who last updated the shared file, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_user: Option<User>,
    /// The timestamp of when the shared file was last updated, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
    /// The content type and type-specific information.
    #[serde(flatten)]
    pub content: FileContent,
}
