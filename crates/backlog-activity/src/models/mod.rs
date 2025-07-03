use backlog_core::{User, identifier::ActivityId};
use backlog_domain_models::Project;
use backlog_project::models::activity::Content;
use backlog_project::models::activity::get_recent_updates_response::Notification;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: ActivityId,
    pub project: Project,
    #[serde(rename = "type")]
    pub type_id: i32,
    pub content: Content,
    pub notifications: Vec<Notification>,
    pub created_user: User,
    pub created: DateTime<Utc>,
}
