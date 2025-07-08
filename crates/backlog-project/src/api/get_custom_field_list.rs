use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use backlog_domain_models::CustomFieldType;
use serde::Serialize;

/// Response type for getting custom field list
pub type GetCustomFieldListResponse = Vec<CustomFieldType>;

/// Parameters for getting the list of custom fields for a project.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/customFields`.
#[derive(Debug, Clone, Serialize)]
pub struct GetCustomFieldListParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetCustomFieldListParams {
    /// Creates new parameters for getting custom field list.
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetCustomFieldListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/customFields", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
