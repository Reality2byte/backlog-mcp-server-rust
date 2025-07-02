use backlog_api_core::IntoRequest;
use backlog_core::identifier::ProjectId;
use serde::{Deserialize, Serialize};

/// Response type for getting space disk usage
pub type GetSpaceDiskUsageResponse = SpaceDiskUsage;

/// Space disk usage information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceDiskUsage {
    pub capacity: i64,
    pub issue: i64,
    pub wiki: i64,
    pub file: i64,
    pub subversion: i64,
    pub git: i64,
    #[serde(rename = "gitLFS")]
    pub git_lfs: i64,
    pub details: Vec<ProjectDiskUsage>,
}

/// Project-specific disk usage information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDiskUsage {
    pub project_id: ProjectId,
    pub issue: i64,
    pub wiki: i64,
    pub document: i64,
    pub file: i64,
    pub subversion: i64,
    pub git: i64,
    #[serde(rename = "gitLFS")]
    pub git_lfs: i64,
}

/// Parameters for getting space disk usage.
///
/// Corresponds to `GET /api/v2/space/diskUsage`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSpaceDiskUsageParams;

impl GetSpaceDiskUsageParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetSpaceDiskUsageParams {
    fn path(&self) -> String {
        "/api/v2/space/diskUsage".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
