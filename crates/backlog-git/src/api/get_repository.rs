use crate::models::Repository;
use backlog_api_core::IntoRequest;
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName};

pub type GetRepositoryResponse = Repository;

#[derive(Debug, Clone)]
pub struct GetRepositoryParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
}

impl GetRepositoryParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
        }
    }
}

impl IntoRequest for GetRepositoryParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}",
            self.project_id_or_key, self.repo_id_or_name
        )
    }
}
