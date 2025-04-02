use serde::{Deserialize, Serialize};

use crate::types;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpaceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_key: Option<String>,
    pub name: String,
    pub owner_id: i32,
    pub lang: String,
    pub timezone: String,
    pub report_send_time: String,
    pub text_formatting_rule: types::TextFormattingRule,
    pub created: String,
    pub updated: String,
}
