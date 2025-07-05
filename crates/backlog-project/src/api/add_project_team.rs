#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, id::TeamId};
#[cfg(feature = "writable")]
use backlog_domain_models::Team;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Parameters for adding a team to a project.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct AddProjectTeamParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(rename = "teamId")]
    pub team_id: TeamId,
}

#[cfg(feature = "writable")]
impl IntoRequest for AddProjectTeamParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/teams", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}

/// Response type for adding a team to a project.
#[cfg(feature = "writable")]
pub type AddProjectTeamResponse = Team;
