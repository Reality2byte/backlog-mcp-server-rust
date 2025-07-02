use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Space notification information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpaceNotification {
    pub content: String,
    pub updated: DateTime<Utc>,
}
