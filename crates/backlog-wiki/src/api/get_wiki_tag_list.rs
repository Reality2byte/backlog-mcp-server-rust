use crate::models::WikiTag;
use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

/// Response type for getting a list of wiki tags.
pub type GetWikiTagListResponse = Vec<WikiTag>;

/// Parameters for getting a list of tags used in wiki pages within a project.
///
/// Corresponds to `GET /api/v2/wikis/tags`.
#[derive(Debug, Clone)]
pub struct GetWikiTagListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetWikiTagListParams {
    /// Create new parameters with the specified project ID or key.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetWikiTagListParams {
    fn path(&self) -> String {
        "/api/v2/wikis/tags".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        // API expects projectIdOrKey as a query parameter
        [("projectIdOrKey", self.project_id_or_key.to_string())]
    }
}
