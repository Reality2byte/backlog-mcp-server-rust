use crate::models::NotificationReason;
use backlog_core::{User, identifier::NotificationId};
use serde::{Deserialize, Serialize};

// Conditionally import and derive JsonSchema
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a notification related to an issue or comment.
///
/// Notifications inform users about activities they are involved in or subscribed to.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct NotificationForComment {
    /// The ID of the notification.
    pub id: NotificationId,
    /// Indicates if the notification has been read by the user.
    pub already_read: bool,
    /// A code indicating the reason for the notification.
    pub reason: NotificationReason,
    /// The user to whom this notification is addressed.
    pub user: User,
    /// Indicates if the resource related to the notification has been read.
    pub resource_already_read: bool,
}
