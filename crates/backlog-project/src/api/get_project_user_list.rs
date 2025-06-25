use backlog_api_core::IntoRequest;
use backlog_core::{ProjectIdOrKey, User};
use serde::Serialize;

/// Response type for getting project user list
pub type GetProjectUserListResponse = Vec<User>;

/// Parameters for getting the list of project members.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/users`.
#[derive(Debug, Clone, Serialize)]
pub struct GetProjectUserListParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetProjectUserListParams {
    /// Creates new parameters for getting project user list.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetProjectUserListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/users", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
