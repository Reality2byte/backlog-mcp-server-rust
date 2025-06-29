use backlog_core::identifier::Identifier;
use backlog_core::{IssueKey, Star, User};
use backlog_issue::models::{Attachment, ExternalFileLink, Issue, SharedFile};
use backlog_project::{Category, IssueType, Milestone, Priority, Resolution, Status};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// AI-friendly representation of an Issue with custom fields as a map
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueResponse {
    // Preserve all original Issue fields
    pub id: backlog_core::identifier::IssueId,
    pub project_id: backlog_core::identifier::ProjectId,
    pub issue_key: IssueKey,
    pub key_id: u32,
    pub issue_type: Box<IssueType>,
    pub summary: String,
    pub description: Option<String>,
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
    // Transform custom fields to a map
    #[serde(rename = "customFields")]
    pub custom_fields: HashMap<String, CustomFieldInfo>,
    // Keep other fields as-is
    pub attachments: Vec<Attachment>,
    pub shared_files: Vec<SharedFile>,
    pub external_file_links: Vec<ExternalFileLink>,
    pub stars: Vec<Star>,
}

/// Custom field information in AI-friendly format
#[derive(Debug, Clone, Serialize)]
pub struct CustomFieldInfo {
    pub id: u32,
    #[serde(rename = "fieldTypeId")]
    pub field_type_id: String,
    pub value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "otherValue")]
    pub other_value: Option<Value>,
}

impl From<Issue> for IssueResponse {
    fn from(issue: Issue) -> Self {
        // Transform custom fields array to map
        let custom_fields = issue
            .custom_fields
            .iter()
            .map(|cf| {
                let field_info = CustomFieldInfo {
                    id: cf.id.value(),
                    field_type_id: cf.field_type_id.to_string(),
                    value: cf.value.clone(),
                    other_value: cf.other_value.clone(),
                };
                (cf.name.clone(), field_info)
            })
            .collect();

        IssueResponse {
            id: issue.id,
            project_id: issue.project_id,
            issue_key: issue.issue_key,
            key_id: issue.key_id,
            issue_type: issue.issue_type,
            summary: issue.summary,
            description: Some(issue.description),
            resolution: issue.resolution,
            priority: issue.priority,
            status: issue.status,
            assignee: issue.assignee,
            category: issue.category,
            versions: issue.versions,
            milestone: issue.milestone,
            start_date: issue.start_date,
            due_date: issue.due_date,
            estimated_hours: issue.estimated_hours,
            actual_hours: issue.actual_hours,
            parent_issue_id: issue.parent_issue_id,
            created_user: issue.created_user,
            created: issue.created,
            updated_user: issue.updated_user,
            updated: issue.updated,
            custom_fields,
            attachments: issue.attachments,
            shared_files: issue.shared_files,
            external_file_links: issue.external_file_links,
            stars: issue.stars,
        }
    }
}

#[cfg(test)]
#[path = "response_transformer_test.rs"]
mod tests;
