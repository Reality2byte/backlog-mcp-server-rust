use backlog_core::identifier::PriorityId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub id: PriorityId,
    pub name: String,
}
