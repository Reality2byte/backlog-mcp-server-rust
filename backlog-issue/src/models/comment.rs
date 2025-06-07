use backlog_core::{
    User,
    identifier::{CommentId, NotificationId, StarId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize}; // Ensured Serialize is here

// Conditionally import and derive JsonSchema
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a comment on a Backlog issue.
///
/// Corresponds to the `Comment` object in the Backlog API.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)] // Removed JsonSchema from here
#[cfg_attr(feature = "schemars", derive(JsonSchema))] // Added conditional JsonSchema
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

/// Represents an entry in the change log associated with a comment.
///
/// This details a specific modification that occurred, such as a change to an issue's
/// status, assignee, or other attributes, recorded as part of a comment.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)] // Removed JsonSchema from here
#[cfg_attr(feature = "schemars", derive(JsonSchema))] // Added conditional JsonSchema
#[serde(rename_all = "camelCase")]
pub struct ChangeLogEntry {
    /// The field that was changed (e.g., "status", "assignee").
    pub field: String,
    /// The new value of the field after the change.
    pub new_value: Option<String>,
    /// The original value of the field before the change.
    pub original_value: Option<String>,
    /// Information about an attachment, if the change log entry relates to one.
    /// The structure of this field can vary, so it's represented as a generic JSON value.
    pub attachment_info: Option<serde_json::Value>,
    /// Information about an attribute, if the change log entry relates to one.
    /// The structure of this field can vary, so it's represented as a generic JSON value.
    pub attribute_info: Option<serde_json::Value>,
    /// Information about a notification, if the change log entry relates to one.
    /// The structure of this field can vary, so it's represented as a generic JSON value.
    pub notification_info: Option<serde_json::Value>,
}

/// Represents a "star" given to a comment.
///
/// Users can star comments to mark them as noteworthy or for quick reference.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)] // Removed JsonSchema from here
#[cfg_attr(feature = "schemars", derive(JsonSchema))] // Added conditional JsonSchema
#[serde(rename_all = "camelCase")]
pub struct Star {
    /// The ID of the star.
    pub id: StarId,
    /// Optional comment associated with the star.
    pub comment: Option<String>,
    /// URL related to the star (often points to the starred item).
    pub url: String,
    /// The user who gave the star.
    pub presenter: User,
    /// The timestamp of when the star was given.
    pub created: DateTime<Utc>,
}

/// Represents a notification related to an issue or comment.
///
/// Notifications inform users about activities they are involved in or subscribed to.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)] // Removed JsonSchema from here
#[cfg_attr(feature = "schemars", derive(JsonSchema))] // Added conditional JsonSchema
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// The ID of the notification.
    pub id: NotificationId,
    /// Indicates if the notification has been read by the user.
    pub already_read: bool,
    /// A code indicating the reason for the notification.
    /// The specific meanings of these codes are defined by the Backlog API.
    pub reason: u8, // Consider making this an enum if specific values are known
    /// The user to whom this notification is addressed.
    pub user: User,
    /// Indicates if the resource related to the notification has been read.
    pub resource_already_read: bool,
}
