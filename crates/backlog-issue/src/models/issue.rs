use crate::models::{Attachment, CustomField, ExternalFileLink, SharedFile};
use backlog_core::{
    IssueKey, Star, User,
    identifier::{IssueId, ProjectId},
};
use backlog_domain_models::{Category, IssueType, Milestone, Priority, Resolution, Status};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: IssueId,
    pub project_id: ProjectId,
    pub issue_key: IssueKey,
    pub key_id: u32,
    pub issue_type: Box<IssueType>,
    pub summary: String,
    pub description: String,
    pub resolution: Option<Box<Resolution>>,
    pub priority: Option<Box<Priority>>,
    pub status: Box<Status>,
    pub assignee: Option<Box<User>>,
    pub category: Vec<Category>,
    pub versions: Vec<Milestone>,
    pub milestone: Vec<Milestone>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub parent_issue_id: Option<i32>,
    pub created_user: Box<User>,
    pub created: String,
    pub updated_user: Option<Box<User>>,
    pub updated: String,
    #[serde(default)]
    pub custom_fields: Vec<CustomField>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub shared_files: Vec<SharedFile>,
    #[serde(default)]
    pub external_file_links: Vec<ExternalFileLink>,
    #[serde(default)]
    pub stars: Vec<Star>,
}
