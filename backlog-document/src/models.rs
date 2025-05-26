use backlog_core::identifier::{ProjectId, StatusId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue; // For the rich text editor JSON

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
//#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String, // UUID
    pub project_id: ProjectId,
    pub title: String,
    pub plain: String,
    pub status_id: StatusId, // Assuming StatusId is a numeric ID type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    pub created_user_id: UserId,
    pub created: DateTime<Utc>,
    pub updated_user_id: UserId,
    pub updated: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>, // Or Vec<Tag> if a Tag struct is defined
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
//#[serde(rename_all = "camelCase")]
pub struct DocumentDetail {
    pub id: String, // UUID
    pub project_id: ProjectId,
    pub title: String,
    pub json: JsonValue, // For ProseMirror or similar rich text JSON
    pub plain: String,
    pub status_id: StatusId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    pub created_user_id: UserId,
    pub created: DateTime<Utc>,
    pub updated_user_id: UserId,
    pub updated: DateTime<Utc>,
    // `tags` field was not in the curl example for single document,
    // but might be present. Adding it as optional for now.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
//#[serde(rename_all = "camelCase")]
pub struct DocumentTreeResponse {
    pub project_id: ProjectId,
    pub active_tree: DocumentTreeNode,
    pub trash_tree: DocumentTreeNode,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
//#[serde(rename_all = "camelCase")]
pub struct DocumentTreeNode {
    pub id: String, // Can be "Active", "Trash", or a document UUID
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

// Placeholder for AttachmentData if needed for download response,
// but download usually returns raw bytes or a stream.
// pub struct AttachmentData { ... }
