use backlog_core::{
    Star, User,
    identifier::{ProjectId, WikiAttachmentId, WikiId, WikiTagId},
};
use backlog_file::models::SharedFile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Wiki {
    pub id: WikiId,
    pub project_id: ProjectId,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WikiTag>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WikiAttachment {
    pub id: WikiAttachmentId,
    pub name: String,
    pub size: u64,
    #[serde(rename = "createdUser")]
    pub created_user: User,
    pub created: DateTime<Utc>,
}

/// Represents the count of wiki pages in a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiCount {
    /// The number of wiki pages
    pub count: u32,
}

/// Represents detailed information about a wiki page, including content, attachments, and stars.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikiDetail {
    /// The unique identifier for the wiki page.
    pub id: WikiId,
    /// The project ID this wiki page belongs to.
    pub project_id: ProjectId,
    /// The name/title of the wiki page.
    pub name: String,
    /// The content of the wiki page (usually in Markdown format).
    pub content: String,
    /// Tags associated with this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WikiTag>,
    /// File attachments associated with this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<WikiAttachment>,
    /// Shared files linked to this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_files: Vec<SharedFile>,
    /// Stars given to this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stars: Vec<Star>,
    /// The user who created this wiki page.
    pub created_user: User,
    /// The timestamp when this wiki page was created.
    pub created: DateTime<Utc>,
    /// The user who last updated this wiki page.
    pub updated_user: User,
    /// The timestamp when this wiki page was last updated.
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WikiTag {
    pub id: WikiTagId,
    pub name: String,
}
