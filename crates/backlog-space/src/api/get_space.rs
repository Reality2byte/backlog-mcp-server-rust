use backlog_api_core::IntoRequest;
use backlog_core::{Language, SpaceKey, TextFormattingRule, identifier::UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Response type for getting space information
pub type GetSpaceResponse = Space;

/// Space information model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub space_key: SpaceKey,
    pub name: String,
    pub owner_id: UserId,
    pub lang: Language,
    pub timezone: String,
    pub report_send_time: String,
    pub text_formatting_rule: TextFormattingRule,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Parameters for getting space information.
///
/// Corresponds to `GET /api/v2/space`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSpaceParams;

impl GetSpaceParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetSpaceParams {
    fn path(&self) -> String {
        "/api/v2/space".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
