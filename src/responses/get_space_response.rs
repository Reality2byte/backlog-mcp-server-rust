use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{SpaceKey, TextFormattingRule};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpaceResponse {
    pub space_key: SpaceKey,
    pub name: String,
    pub owner_id: i64,
    pub lang: String,
    pub timezone: String,
    pub report_send_time: String,
    pub text_formatting_rule: TextFormattingRule,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
