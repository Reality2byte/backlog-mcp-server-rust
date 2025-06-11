use crate::models::CustomFieldTypeId;
use backlog_core::identifier::{CustomFieldId, IssueTypeId, ProjectId};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a custom field associated with an issue.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldType {
    /// The ID of the custom field.
    pub id: CustomFieldId,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// The field type ID.
    pub type_id: CustomFieldTypeId,
    /// The name of the custom field.
    pub name: String,
    /// Whether the custom field is required.
    pub required: bool,
    /// Type ID to enable Custom fields. Empty means enable for all issue types.
    pub applicable_issue_types: Vec<IssueTypeId>,
}
