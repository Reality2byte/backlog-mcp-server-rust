use crate::models::Repository;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;

pub type GetRepositoryListResponse = Vec<Repository>;

#[derive(Debug, Clone)]
pub struct GetRepositoryListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetRepositoryListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetRepositoryListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories",
            self.project_id_or_key
        )
    }
}
