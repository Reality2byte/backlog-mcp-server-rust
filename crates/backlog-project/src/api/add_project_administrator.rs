#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, User};
#[cfg(feature = "writable")]
use serde::Serialize;

/// Parameters for adding a project administrator.
///
/// Corresponds to `POST /api/v2/projects/:projectIdOrKey/administrators`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct AddProjectAdministratorParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[cfg(feature = "writable")]
impl AddProjectAdministratorParams {
    /// Creates a new instance of `AddProjectAdministratorParams` with required parameters.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, user_id: u32) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            user_id,
        }
    }
}

/// Type alias for the response of `add_project_administrator` API.
#[cfg(feature = "writable")]
pub type AddProjectAdministratorResponse = User;

#[cfg(feature = "writable")]
impl IntoRequest for AddProjectAdministratorParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/administrators", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
