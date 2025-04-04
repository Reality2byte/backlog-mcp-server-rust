use serde::{Deserialize, Serialize};

use super::{Category, IssueType, Milestone, Priority, Resolution, Status, User};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: i32,
    pub project_id: i32,
    pub issue_key: String,
    pub key_id: i32,
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
    //pub custom_fields: Vec<CustomFieldValue>,
    //pub attachments: Vec<Attachment>,
    //pub shared_files: Vec<SharedFile>,
    //pub stars: Vec<Star>,
}
