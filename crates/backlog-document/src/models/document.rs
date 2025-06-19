use crate::models::tag::DocumentTag;
use backlog_core::{
    User,
    identifier::{DocumentId, ProjectId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
