use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use derive_builder::Builder;

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
