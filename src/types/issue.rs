use serde::{Deserialize, Serialize};

use super::{IssueType, User};

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
    //pub resolution: Box<ProjectWritesImplicitsPeriodResolutionJsonWrites>,
    //pub priority: Box<ProjectWritesImplicitsPeriodPriorityJsonWrites>,
    //pub status: Box<StatusDtoWrites>,
    pub assignee: Box<User>,
    //pub category: Vec<ProjectWritesImplicitsPeriodCategoryJsonWrites>,
    //pub versions: Vec<ProjectWritesImplicitsPeriodVersionJsonWrites>,
    //pub milestone: Vec<ProjectWritesImplicitsPeriodVersionJsonWrites>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub parent_issue_id: Option<i32>,
    pub created_user: Box<User>,
    pub created: String,
    pub updated_user: Box<User>,
    pub updated: String,
    //pub custom_fields: Vec<CustomFieldValue>,
    //pub attachments: Vec<IssueWritesImplicitsPeriodAttachmentJsonWrites>,
    //pub shared_files: Vec<SharedFile>,
    //pub stars: Vec<Star>,
}
