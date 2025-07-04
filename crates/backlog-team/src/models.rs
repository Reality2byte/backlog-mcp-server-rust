mod team_response;

pub use team_response::TeamResponse;

use backlog_core::{id::UserId, Language, Role};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Team member representation with extended user information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    pub id: UserId,
    pub user_id: Option<String>,
    pub name: String,
    pub role_type: Role,
    pub lang: Option<Language>,
    pub mail_address: String,
    pub last_login_time: Option<DateTime<Utc>>,
    /// Extra fields that are present in team member response but not in the standard User struct
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
