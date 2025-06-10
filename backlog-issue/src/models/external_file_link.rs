use backlog_core::User;
use serde::{Deserialize, Serialize};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a link to an external file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ExternalFileLink {
    /// The ID of the external file link.
    pub id: u32,
    /// The type of the external file link.
    pub r#type: String,
    /// The URL of the external file.
    pub url: String,
    /// The title of the external file link.
    pub title: String,
    /// The user who created the link.
    pub created_user: User,
    /// The timestamp of when the link was created.
    pub created: String,
    /// The user who last updated the link, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_user: Option<User>,
    /// The timestamp of when the link was last updated, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}
