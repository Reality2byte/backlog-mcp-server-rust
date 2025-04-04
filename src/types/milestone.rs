use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_date: DateTime<chrono::Utc>,
    pub release_due_date: DateTime<chrono::Utc>,
    pub archived: bool,
    pub display_order: i32,
}
