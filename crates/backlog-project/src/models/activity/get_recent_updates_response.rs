use serde::{Deserialize, Serialize};

#[allow(deprecated)]
pub type GetRecentUpdatesResponse = Vec<Activity>;

#[allow(deprecated)]
#[deprecated(since = "0.2.0", note = "Use backlog_core::activity::Activity instead")]
pub type Activity = backlog_core::activity::Activity;

#[deprecated(since = "0.2.0", note = "Use backlog_core::activity::Content instead")]
pub use backlog_core::activity::Content;

#[deprecated(
    since = "0.2.0",
    note = "Use backlog_core::activity::GroupProjectActivity instead"
)]
pub use backlog_core::activity::GroupProjectActivity;

#[deprecated(since = "0.2.0", note = "Use backlog_core::activity::Comment instead")]
pub use backlog_core::activity::Comment;

#[deprecated(since = "0.2.0", note = "Use backlog_core::activity::Change instead")]
pub use backlog_core::activity::Change;

#[deprecated(
    since = "0.2.0",
    note = "Use backlog_core::activity::EmptyNotification instead"
)]
pub type Notification = backlog_core::activity::EmptyNotification;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NulabAccount {
    pub nulab_id: String,
    pub name: String,
    pub unique_id: String,
}

// Removed deserialize_comment function - now using backlog_core's implementation
