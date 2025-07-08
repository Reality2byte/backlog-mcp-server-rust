#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use backlog_core::identifier::CustomFieldId;
#[cfg(feature = "writable")]
use backlog_domain_models::CustomFieldType;

/// Response type for deleting a custom field
#[cfg(feature = "writable")]
pub type DeleteCustomFieldResponse = CustomFieldType;

/// Parameters for deleting a custom field from a project.
///
/// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/customFields/:id`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteCustomFieldParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub custom_field_id: CustomFieldId,
}

#[cfg(feature = "writable")]
impl DeleteCustomFieldParams {
    /// Creates new parameters for deleting a custom field.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        custom_field_id: impl Into<CustomFieldId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            custom_field_id: custom_field_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteCustomFieldParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/customFields/{}",
            self.project_id_or_key, self.custom_field_id
        )
    }
}
