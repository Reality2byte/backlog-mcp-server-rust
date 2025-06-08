use backlog_core::identifier::{CategoryId, ProjectId};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a category in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: CategoryId,
    pub project_id: ProjectId,
    pub name: String,
    pub display_order: i32,
}
