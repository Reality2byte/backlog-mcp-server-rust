use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use backlog_core::identifier::WatchingId;
use backlog_issue::Issue;

/// Type of watching target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WatchingType {
    Issue,
}

/// Watching record for issue tracking
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watching {
    pub id: WatchingId,
    #[serde(default)]
    pub resource_already_read: bool,
    pub note: Option<String>,
    #[serde(rename = "type")]
    pub watching_type: WatchingType,
    pub issue: Option<Issue>,
    pub last_content_updated: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    /// This field is only returned by the Get Watching API
    #[serde(default)]
    pub already_read: bool,
}

/// Count of watchings
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WatchingCount {
    /// The number of watchings
    pub count: u32,
}

/// Response type for get watching list API
pub type GetWatchingListResponse = Vec<Watching>;

/// Response type for get watching count API
pub type GetWatchingCountResponse = WatchingCount;

#[cfg(test)]
#[path = "models_test.rs"]
mod tests;
