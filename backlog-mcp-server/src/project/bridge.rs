use crate::access_control::AccessControl;
use crate::error::Result;
use crate::project::request::{
    GetCustomFieldListRequest, GetPrioritiesRequest, GetProjectIssueTypesRequest,
    GetProjectStatusListRequest,
};
use backlog_api_client::IssueType;
use backlog_api_client::ProjectIdOrKey; // From backlog-core, re-exported by backlog-api-client
use backlog_api_client::client::BacklogApiClient;
use backlog_project::Priority;
use backlog_project::Status; // Specific model from backlog-project
use std::sync::Arc;
use tokio::sync::Mutex;

/// Helper function to implement the get_project_status_list tool.
pub(crate) async fn get_project_status_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetProjectStatusListRequest,
    access_control: &AccessControl,
) -> Result<Vec<Status>> {
    // Parse the project_id_or_key from the request string.
    // This will use From<CoreError> for Error if parsing fails.
    let project_id = req.project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&project_id, &client_guard)
        .await?;
    // This will use From<ApiError> for Error if the API call fails.
    let params = backlog_project::GetStatusListParams::new(project_id);
    let statuses = client_guard.project().get_status_list(params).await?;
    Ok(statuses)
}

/// Helper function to implement the get_project_issue_types tool.
pub(crate) async fn get_project_issue_types_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetProjectIssueTypesRequest,
    access_control: &AccessControl,
) -> Result<Vec<IssueType>> {
    let project_id = req.project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&project_id, &client_guard)
        .await?;
    let params = backlog_project::GetIssueTypeListParams::new(project_id);
    let issue_types = client_guard.project().get_issue_type_list(params).await?;
    Ok(issue_types)
}

/// Helper function to implement the get_priorities tool.
pub(crate) async fn get_priorities_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    _req: GetPrioritiesRequest,
) -> Result<Vec<Priority>> {
    let client_guard = client.lock().await;
    let priorities = client_guard.project().get_priority_list().await?;
    Ok(priorities)
}

/// Helper function to implement the get_custom_field_list tool.
pub(crate) async fn get_custom_field_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetCustomFieldListRequest,
    access_control: &AccessControl,
) -> Result<Vec<serde_json::Value>> {
    use backlog_core::identifier::Identifier;
    use backlog_project::GetCustomFieldListParams;
    use serde_json::json;

    let project_id = req.project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&project_id, &client_guard)
        .await?;

    // Get custom field list from project API
    let params = GetCustomFieldListParams::new(project_id);
    let custom_fields = client_guard.project().get_custom_field_list(params).await?;

    // Format the response with examples
    let formatted_fields = custom_fields
        .into_iter()
        .map(|field| {
            let (type_id, items, example) = generate_custom_field_info(&field);
            json!({
                "id": field.id.value(),
                "name": field.name,
                "typeId": type_id,
                "description": field.description,
                "required": field.required,
                "applicableIssueTypes": field.applicable_issue_types,
                "items": items,
                "example": example,
            })
        })
        .collect();

    Ok(formatted_fields)
}

fn generate_custom_field_info(
    field: &backlog_issue::CustomFieldType,
) -> (i64, Option<serde_json::Value>, serde_json::Value) {
    use backlog_core::identifier::Identifier;
    use backlog_issue::models::CustomFieldSettings;
    use serde_json::json;

    match &field.settings {
        CustomFieldSettings::Text => (1, None, json!("Sample text value")),
        CustomFieldSettings::TextArea => {
            (2, None, json!("Multi-line text\nSecond line\nThird line"))
        }
        CustomFieldSettings::Numeric(_) => (3, None, json!(123.45)),
        CustomFieldSettings::Date(_) => (4, None, json!("2024-12-25")),
        CustomFieldSettings::SingleList(settings) => {
            let items_json = Some(json!(
                settings
                    .items
                    .iter()
                    .map(|item| {
                        json!({
                            "id": item.id.value(),
                            "name": item.name,
                            "displayOrder": item.display_order
                        })
                    })
                    .collect::<Vec<_>>()
            ));

            let example = if let Some(first_item) = settings.items.first() {
                json!(first_item.name.clone())
            } else {
                json!("Select an option")
            };

            (5, items_json, example)
        }
        CustomFieldSettings::MultipleList(settings) => {
            let items_json = Some(json!(
                settings
                    .items
                    .iter()
                    .map(|item| {
                        json!({
                            "id": item.id.value(),
                            "name": item.name,
                            "displayOrder": item.display_order
                        })
                    })
                    .collect::<Vec<_>>()
            ));

            let example: Vec<_> = settings
                .items
                .iter()
                .take(2)
                .map(|item| item.name.clone())
                .collect();

            (6, items_json, json!(example))
        }
        CustomFieldSettings::Checkbox(settings) => {
            let items_json = Some(json!(
                settings
                    .items
                    .iter()
                    .map(|item| {
                        json!({
                            "id": item.id.value(),
                            "name": item.name,
                            "displayOrder": item.display_order
                        })
                    })
                    .collect::<Vec<_>>()
            ));

            let example: Vec<_> = settings
                .items
                .iter()
                .take(2)
                .map(|item| item.name.clone())
                .collect();

            (7, items_json, json!(example))
        }
        CustomFieldSettings::Radio(settings) => {
            let items_json = Some(json!(
                settings
                    .items
                    .iter()
                    .map(|item| {
                        json!({
                            "id": item.id.value(),
                            "name": item.name,
                            "displayOrder": item.display_order
                        })
                    })
                    .collect::<Vec<_>>()
            ));

            let example = if let Some(first_item) = settings.items.first() {
                json!(first_item.name.clone())
            } else {
                json!("Select an option")
            };

            (8, items_json, example)
        }
    }
}
