use backlog_core::identifier::PriorityId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a priority in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub id: PriorityId,
    pub name: String,
}
