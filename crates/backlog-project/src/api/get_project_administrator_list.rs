use backlog_api_core::IntoRequest;
use backlog_core::{ProjectIdOrKey, User};

/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/administrators`.
///
/// Type alias for the response of `get_project_administrator_list` API.
pub type GetProjectAdministratorListResponse = Vec<User>;

/// Parameters for getting list of project administrators.
#[derive(Debug, Clone)]
pub struct GetProjectAdministratorListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetProjectAdministratorListParams {
    /// Creates a new instance of `GetProjectAdministratorListParams` with required parameters.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetProjectAdministratorListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/administrators", self.project_id_or_key)
    }
}
