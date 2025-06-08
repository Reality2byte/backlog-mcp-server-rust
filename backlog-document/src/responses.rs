use crate::models::DocumentTreeNode;
use backlog_core::identifier::ProjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTreeResponse {
    pub project_id: ProjectId,
    pub active_tree: DocumentTreeNode,
    pub trash_tree: DocumentTreeNode,
}
