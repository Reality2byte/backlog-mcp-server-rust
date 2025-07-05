#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, id::TeamId};
#[cfg(feature = "writable")]
use backlog_domain_models::Team;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Parameters for deleting a team from a project.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteProjectTeamParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(rename = "teamId")]
    pub team_id: TeamId,
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteProjectTeamParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/teams", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}

/// Response type for deleting a team from a project.
#[cfg(feature = "writable")]
pub type DeleteProjectTeamResponse = Team;
