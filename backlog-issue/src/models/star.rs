use backlog_core::{User, identifier::StarId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Conditionally import and derive JsonSchema
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a "star" given to a comment.
///
/// Users can star comments to mark them as noteworthy or for quick reference.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Star {
    /// The ID of the star.
    pub id: StarId,
    /// Optional comment associated with the star.
    pub comment: Option<String>,
    /// URL related to the star (often points to the starred item).
    pub url: String,
    /// The user who gave the star.
    pub presenter: User,
    /// The timestamp of when the star was given.
    pub created: DateTime<Utc>,
}
