use backlog_core::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub hook_url: String,
    pub all_event: bool,
    pub activity_type_ids: Vec<u32>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
}
