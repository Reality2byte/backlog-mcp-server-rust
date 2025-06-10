use crate::models::{ChangeLogEntry, Notification};
use backlog_core::{Star, User, identifier::CommentId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Conditionally import and derive JsonSchema
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a comment on a Backlog issue.
///
/// Corresponds to the `Comment` object in the Backlog API.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    /// The ID of the comment.
    pub id: CommentId,
    /// The content of the comment.
    pub content: Option<String>,
    /// A list of changes made in this comment.
    #[serde(default)]
    pub change_log: Vec<ChangeLogEntry>,
    /// The user who created the comment.
    pub created_user: User,
    /// The timestamp of when the comment was created.
    pub created: DateTime<Utc>,
    /// The timestamp of when the comment was last updated.
    pub updated: DateTime<Utc>,
    /// A list of stars given to this comment.
    #[serde(default)]
    pub stars: Vec<Star>,
    /// A list of notifications related to this comment.
    #[serde(default)]
    pub notifications: Vec<Notification>,
}
