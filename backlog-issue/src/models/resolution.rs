use backlog_core::identifier::ResolutionId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub id: ResolutionId,
    pub name: String,
}
