use backlog_core::identifier::{IssueTypeId, ProjectId};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents an issue type in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    pub id: IssueTypeId,
    pub project_id: ProjectId,
    pub name: String,
    pub color: String,
    pub display_order: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
}