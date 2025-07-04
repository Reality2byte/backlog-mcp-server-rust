use backlog_core::{User, id::TeamId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub members: Vec<User>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
}
