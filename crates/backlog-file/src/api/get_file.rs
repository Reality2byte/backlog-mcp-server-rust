use backlog_api_core::{HttpMethod, IntoDownloadRequest, IntoRequest};
use backlog_core::{
    ProjectIdOrKey,
    identifier::{Identifier, SharedFileId},
};
use derive_builder::Builder;

/// Parameters for downloading a shared file.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/files/:sharedFileId`.
#[derive(Builder, Debug, Clone)]
pub struct GetFileParams {
    /// The project ID or key.
    pub project_id_or_key: ProjectIdOrKey,
    /// The shared file ID.
    pub shared_file_id: SharedFileId,
}

impl GetFileParams {
    /// Creates a new instance with the required parameters.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, shared_file_id: SharedFileId) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            shared_file_id,
        }
    }
}

impl IntoRequest for GetFileParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/files/{}",
            self.project_id_or_key,
            self.shared_file_id.value()
        )
    }
}

impl IntoDownloadRequest for GetFileParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/files/{}",
            self.project_id_or_key,
            self.shared_file_id.value()
        )
    }
}
