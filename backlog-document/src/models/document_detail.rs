use backlog_core::{DocumentId, User, identifier::ProjectId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::models::{attachment::DocumentAttachment, tag::DocumentTag};

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
