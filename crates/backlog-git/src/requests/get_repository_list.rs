use backlog_api_core::{Error as ApiError, IntoRequest, Result};
use backlog_core::ProjectIdOrKey;
use derive_builder::Builder;
use reqwest::Client as ReqwestClient;
use url::Url;

/// Parameters for getting repository list.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetRepositoryListParams {
    /// The project ID or key.
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetRepositoryListParams {
    /// Creates a new instance with the required project parameter.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetRepositoryListParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories",
            self.project_id_or_key
        )
    }
    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        self.get(client, base_url, &())
    }
}
