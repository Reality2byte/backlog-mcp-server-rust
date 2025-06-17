use backlog_core::identifier::{MilestoneId, ProjectId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a milestone in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    pub id: MilestoneId,
    pub project_id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub release_due_date: Option<DateTime<Utc>>,
    pub archived: bool,
    pub display_order: Option<i32>,
}