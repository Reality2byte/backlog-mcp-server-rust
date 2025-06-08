use backlog_core::{
    DocumentId, User,
    identifier::{DocumentAttachmentId, ProjectId, StatusId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: DocumentId,
    pub project_id: ProjectId,
    pub title: String,
    pub plain: String,
    pub status_id: i32, // Assuming status_id is an integer, adjust if it's a different type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<DocumentTag>, // Or Vec<Tag> if a Tag struct is defined
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentDetail {
    pub id: DocumentId,
    pub project_id: ProjectId,
    pub title: String,
    pub json: JsonValue, // assuming ProseMirror JSON
    pub plain: String,
    pub status_id: i32, // Document's own status, not project status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<DocumentAttachment>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<DocumentTag>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentAttachment {
    pub id: DocumentAttachmentId,
    pub name: String,
    pub size: u64,
    pub created_user: User,
    pub created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct DocumentTag {
    pub id: u32, // Assuming ID is a numeric type
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DocumentTreeResponse {
    pub project_id: ProjectId,
    pub active_tree: DocumentTreeNode,
    pub trash_tree: DocumentTreeNode,
}

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
