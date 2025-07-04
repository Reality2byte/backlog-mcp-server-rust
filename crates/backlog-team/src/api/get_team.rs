use crate::models::TeamResponse;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::id::TeamId;
use serde::{Deserialize, Serialize};

/// Response type for getting a team.
pub type GetTeamResponse = TeamResponse;

/// Parameters for getting a team.
///
/// # Required Permissions
/// - Administrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamParams {
    pub team_id: TeamId,
}

impl IntoRequest for GetTeamParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/teams/{}", self.team_id)
    }
}
