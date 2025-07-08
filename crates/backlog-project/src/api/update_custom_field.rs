#[cfg(feature = "writable")]
use backlog_api_core::IntoRequest;
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{
    Date, ProjectIdOrKey,
    identifier::{CustomFieldId, IssueTypeId},
};
#[cfg(feature = "writable")]
use backlog_domain_models::CustomFieldType;

/// Response type for updating a custom field
#[cfg(feature = "writable")]
pub type UpdateCustomFieldResponse = CustomFieldType;

/// Parameters for updating a custom field in a project.
///
/// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/customFields/:id`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct UpdateCustomFieldParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[form(skip)]
    pub custom_field_id: CustomFieldId,
    pub name: Option<String>,
    #[form(array, name = "applicableIssueTypes")]
    pub applicable_issue_types: Option<Vec<IssueTypeId>>,
    pub description: Option<String>,
    pub required: Option<bool>,
    // Numeric field specific parameters
    pub min: Option<f64>,
    pub max: Option<f64>,
    #[form(name = "initialValue")]
    pub initial_value: Option<f64>,
    pub unit: Option<String>,
    // Date field specific parameters
    #[form(name = "min")]
    pub min_date: Option<Date>,
    #[form(name = "max")]
    pub max_date: Option<Date>,
    #[form(name = "initialValueType")]
    pub initial_value_type: Option<i32>,
    #[form(name = "initialDate")]
    pub initial_date: Option<Date>,
    #[form(name = "initialShift")]
    pub initial_shift: Option<i32>,
    // List field specific parameters
    #[form(array, name = "items")]
    pub items: Option<Vec<String>>,
    #[form(name = "allowInput")]
    pub allow_input: Option<bool>,
    #[form(name = "allowAddItem")]
    pub allow_add_item: Option<bool>,
}

#[cfg(feature = "writable")]
impl UpdateCustomFieldParams {
    /// Creates new parameters for updating a custom field.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        custom_field_id: impl Into<CustomFieldId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            custom_field_id: custom_field_id.into(),
            name: None,
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: None,
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Sets the name of the custom field.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the applicable issue types.
    pub fn with_applicable_issue_types(mut self, issue_types: Vec<IssueTypeId>) -> Self {
        self.applicable_issue_types = Some(issue_types);
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether the field is required.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Sets numeric field parameters.
    pub fn with_numeric_settings(
        mut self,
        min: Option<f64>,
        max: Option<f64>,
        initial_value: Option<f64>,
        unit: Option<String>,
    ) -> Self {
        self.min = min;
        self.max = max;
        self.initial_value = initial_value;
        self.unit = unit;
        self
    }

    /// Sets date field parameters.
    pub fn with_date_settings(
        mut self,
        min: Option<Date>,
        max: Option<Date>,
        initial_value_type: Option<i32>,
        initial_date: Option<Date>,
        initial_shift: Option<i32>,
    ) -> Self {
        self.min_date = min;
        self.max_date = max;
        self.initial_value_type = initial_value_type;
        self.initial_date = initial_date;
        self.initial_shift = initial_shift;
        self
    }

    /// Sets list field parameters.
    pub fn with_list_settings(
        mut self,
        items: Option<Vec<String>>,
        allow_input: Option<bool>,
        allow_add_item: Option<bool>,
    ) -> Self {
        self.items = items;
        self.allow_input = allow_input;
        self.allow_add_item = allow_add_item;
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateCustomFieldParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/customFields/{}",
            self.project_id_or_key, self.custom_field_id
        )
    }

    fn method(&self) -> backlog_api_core::HttpMethod {
        backlog_api_core::HttpMethod::Patch
    }

    fn to_form(&self) -> impl serde::Serialize {
        Vec::<(String, String)>::from(self)
    }
}
