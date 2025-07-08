use backlog_core::User;
use backlog_core::activity::NotificationReason;
use backlog_core::identifier::NotificationId;
use backlog_domain_models::Project;
use backlog_issue::{Comment, Issue};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: NotificationId,
    pub already_read: bool,
    pub reason: NotificationReason,
    pub resource_already_read: bool,
    pub project: Project,
    pub issue: Option<Issue>,
    pub comment: Option<Comment>,
    pub pull_request: Option<serde_json::Value>,
    pub pull_request_comment: Option<serde_json::Value>,
    pub sender: User,
    pub created: DateTime<Utc>,
}
