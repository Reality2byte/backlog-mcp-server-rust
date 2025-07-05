use backlog_api_core::IntoDownloadRequest;
use backlog_core::{id::TeamId, identifier::Identifier};

/// Parameters for getting a team icon.
///
/// Corresponds to `GET /api/v2/teams/:teamId/icon`.
///
/// # Required Permissions
/// - All permissions
#[derive(Debug, Clone)]
pub struct GetTeamIconParams {
    /// Team ID.
    pub team_id: TeamId,
}

impl IntoDownloadRequest for GetTeamIconParams {
    fn path(&self) -> String {
        format!("/api/v2/teams/{}/icon", self.team_id.value())
    }
}
