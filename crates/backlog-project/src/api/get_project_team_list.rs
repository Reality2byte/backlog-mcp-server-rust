use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use backlog_domain_models::Team;
use serde::Serialize;

/// Parameters for getting the list of teams in a project.
#[derive(Debug, Clone, Serialize)]
pub struct GetProjectTeamListParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
}

impl IntoRequest for GetProjectTeamListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/teams", self.project_id_or_key)
    }
}

/// Response type for getting the list of teams in a project.
pub type GetProjectTeamListResponse = Vec<Team>;
