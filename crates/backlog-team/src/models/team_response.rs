use crate::models::TeamMember;
use backlog_core::{id::TeamId, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Response structure for team API that includes TeamMember instead of User
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamResponse {
    pub id: TeamId,
    pub name: String,
    pub members: Vec<TeamMember>,
    pub created_user: User,
    pub created: DateTime<Utc>,
    pub updated_user: User,
    pub updated: DateTime<Utc>,
}
