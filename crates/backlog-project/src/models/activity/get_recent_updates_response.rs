use backlog_core::User;
use backlog_domain_models::Project;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type GetRecentUpdatesResponse = Vec<Activity>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: i64,
    pub project: Project,
    #[serde(rename = "type")]
    pub type_id: i32,
    pub content: Content,
    pub notifications: Vec<Notification>,
    pub created_user: User,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub key_id: Option<i64>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub comment: Option<Comment>,
    pub changes: Option<Vec<Change>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Change {
    pub field: String,
    pub new_value: String,
    pub old_value: String,
    #[serde(rename = "type")]
    pub change_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    // Empty as per API spec example
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NulabAccount {
    pub nulab_id: String,
    pub name: String,
    pub unique_id: String,
}
