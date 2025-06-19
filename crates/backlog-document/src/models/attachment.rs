use backlog_core::{User, identifier::DocumentAttachmentId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DocumentAttachment {
    pub id: DocumentAttachmentId,
    pub name: String,
    pub size: u64,
    pub created_user: User,
    pub created: DateTime<Utc>,
}
