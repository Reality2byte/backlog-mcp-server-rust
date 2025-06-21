use backlog_core::identifier::WikiTagId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WikiTag {
    pub id: WikiTagId,
    pub name: String,
}
