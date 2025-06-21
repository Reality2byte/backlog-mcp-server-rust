use backlog_api_core::{Error as ApiError, IntoRequest, Result};
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName};
use derive_builder::Builder;
use reqwest::Client as ReqwestClient;
use url::Url;

/// Parameters for getting a single repository.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetRepositoryParams {
    /// The project ID or key.
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    pub repo_id_or_name: RepositoryIdOrName,
}

impl GetRepositoryParams {
    /// Creates a new instance with the required parameters.
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
    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}",
            self.project_id_or_key, self.repo_id_or_name
        );
        self.get(client, base_url, path, &())
    }
}
