use backlog_core::{
    IssueKey, User,
    identifier::{CategoryId, IssueId, PriorityId, ProjectId, ResolutionId},
};
use backlog_project::{IssueType, Milestone, Status};
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub id: ResolutionId,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub id: PriorityId,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub display_order: i32,
}
