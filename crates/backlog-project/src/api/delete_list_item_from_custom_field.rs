#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use backlog_core::identifier::{CustomFieldId, CustomFieldItemId};
#[cfg(feature = "writable")]
use backlog_domain_models::CustomFieldType;

/// Response type for deleting a list item from a custom field.
#[cfg(feature = "writable")]
pub type DeleteListItemFromCustomFieldResponse = CustomFieldType;

/// Parameters for deleting a list item from a custom field.
///
/// Only administrators and project administrators can call this API.
/// Calling API fails if specified custom field's type is not a list.
///
/// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/customFields/:id/items/:itemId`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteListItemFromCustomFieldParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub custom_field_id: CustomFieldId,
    pub list_item_id: CustomFieldItemId,
}

#[cfg(feature = "writable")]
impl DeleteListItemFromCustomFieldParams {
    /// Creates new parameters for deleting a list item from a custom field.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        custom_field_id: impl Into<CustomFieldId>,
        list_item_id: impl Into<CustomFieldItemId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            custom_field_id: custom_field_id.into(),
            list_item_id: list_item_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteListItemFromCustomFieldParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/customFields/{}/items/{}",
            self.project_id_or_key, self.custom_field_id, self.list_item_id
        )
    }
}
