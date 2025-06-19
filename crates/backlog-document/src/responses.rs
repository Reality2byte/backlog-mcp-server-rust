use crate::models::DocumentTreeRootNode; // DocumentTreeNode is no longer directly used here
use backlog_core::identifier::ProjectId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentTreeResponse {
    pub project_id: ProjectId,
    pub active_tree: DocumentTreeRootNode,
    pub trash_tree: DocumentTreeRootNode,
}
