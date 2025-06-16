use backlog_core::{User, identifier::SharedFileId};
use chrono::{DateTime, Utc};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents a shared file in Backlog.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SharedFile {
    /// The ID of the shared file.
    pub id: SharedFileId,
    /// The project ID of the shared file.
    pub project_id: u32,
    /// The type of the shared file (e.g., "file", "directory").
    pub r#type: String,
    /// The directory path of the shared file.
    pub dir: String,
    /// The name of the shared file.
    pub name: String,
    /// The size of the shared file in bytes. Folders may not have a size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// The user who created the shared file.
    pub created_user: User,
    /// The timestamp of when the shared file was created.
    pub created: String,
    /// The user who last updated the shared file, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_user: Option<User>,
    /// The timestamp of when the shared file was last updated, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,
}
