use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;

// Re-export from backlog-space for convenience
pub use backlog_space::api::ProjectDiskUsage;

/// Response type for getting project disk usage
pub type GetProjectDiskUsageResponse = ProjectDiskUsage;

/// Parameters for getting project disk usage.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/diskUsage`.
#[derive(Debug, Clone)]
pub struct GetProjectDiskUsageParams {
    project_id_or_key: ProjectIdOrKey,
}

impl GetProjectDiskUsageParams {
    /// Creates a new instance.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetProjectDiskUsageParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/diskUsage", self.project_id_or_key)
    }
}
