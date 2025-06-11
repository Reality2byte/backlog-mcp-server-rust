use backlog_core::Date;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct InitialDate {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
}
