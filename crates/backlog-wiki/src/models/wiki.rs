use crate::models::WikiTag;
use backlog_core::{
    User,
    identifier::{ProjectId, WikiId},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Wiki {
    pub id: WikiId,
    pub project_id: ProjectId,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WikiTag>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
}
