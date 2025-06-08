use backlog_core::{DocumentId, identifier::StatusId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTreeNode {
    pub id: DocumentId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // Document title, or None for root tree nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<u32>, // Type is unclear from curl, assuming u32 for now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "emojiType")]
    pub emoji_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<DocumentTreeNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "statusId")] // API response uses statusId
    pub status_id: Option<StatusId>,
}
