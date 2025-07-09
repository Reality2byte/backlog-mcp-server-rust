#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;

#[cfg(feature = "writable")]
use backlog_domain_models::Project;

#[cfg(feature = "writable")]
pub type DeleteProjectResponse = Project;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteProjectParams {
    project_id_or_key: ProjectIdOrKey,
}

#[cfg(feature = "writable")]
impl DeleteProjectParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteProjectParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}", self.project_id_or_key)
    }
}
