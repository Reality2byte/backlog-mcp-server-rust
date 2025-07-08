use serde::{Deserialize, Serialize};

/// Represents a Nulab account
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NulabAccount {
    pub nulab_id: String,
    pub name: String,
    pub unique_id: String,
}
