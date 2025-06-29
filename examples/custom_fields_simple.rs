//! Simple example of using custom fields with Backlog issues
//!
//! This example shows the most common use case: adding text and date custom fields

use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::{CustomFieldId, IssueTypeId, PriorityId, ProjectId};
use backlog_issue::api::AddIssueParamsBuilder;
use backlog_issue::models::CustomFieldInput;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;
    let client = BacklogApiClient::new(&base_url)?.with_api_key(&api_key);

    // Create custom fields
    let mut custom_fields = HashMap::new();

    // Add a text custom field (replace 1 with your actual custom field ID)
    custom_fields.insert(
        CustomFieldId::new(1),
        CustomFieldInput::Text("Project ABC".to_string()),
    );

    // Add a date custom field (replace 2 with your actual custom field ID)
    custom_fields.insert(
        CustomFieldId::new(2),
        CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap()),
    );

    // Create issue with custom fields
    let params = AddIssueParamsBuilder::default()
        .project_id(ProjectId::new(1)) // Replace with your project ID
        .summary("Task with custom fields".to_string())
        .issue_type_id(IssueTypeId::new(1))
        .priority_id(PriorityId::new(2))
        .custom_fields(custom_fields)
        .build()?;

    let issue = client.issue().add_issue(params).await?;

    println!("Created issue: {}", issue.issue_key);

    // Display custom fields
    println!("Custom fields: {:?}", issue.custom_fields);

    Ok(())
}
