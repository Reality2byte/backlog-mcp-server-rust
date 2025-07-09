#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, User};
#[cfg(feature = "writable")]
use serde::Serialize;

/// Parameters for deleting a project administrator.
///
/// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/administrators`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteProjectAdministratorParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[cfg(feature = "writable")]
impl DeleteProjectAdministratorParams {
    /// Creates a new instance of `DeleteProjectAdministratorParams` with required parameters.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, user_id: u32) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            user_id,
        }
    }
}

/// Type alias for the response of `delete_project_administrator` API.
#[cfg(feature = "writable")]
pub type DeleteProjectAdministratorResponse = User;

#[cfg(feature = "writable")]
impl IntoRequest for DeleteProjectAdministratorParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/administrators", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
