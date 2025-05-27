use backlog_api_client::client::BacklogApiClient;
use backlog_core::{IssueKey, ProjectIdOrKey};
use backlog_issue::{Issue, Milestone, requests::GetIssueListParamsBuilder};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::{Error as McpError, Result};

pub async fn get_issue_details(
    client: Arc<Mutex<BacklogApiClient>>,
    issue_key: String,
) -> Result<Issue> {
    let client_guard = client.lock().await; // Renamed for clarity
    let parsed_issue_key = IssueKey::from_str(issue_key.trim())?;
    let issue = client_guard
        .issue()
        .get_issue(parsed_issue_key.clone())
        .await?;
    Ok(issue)
}

pub async fn get_version_milestone_list_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    project_id_or_key_str: String,
) -> Result<Vec<Milestone>> {
    let client_guard = client.lock().await;
    let proj_id_or_key = ProjectIdOrKey::from_str(project_id_or_key_str.trim())?;
    let versions = client_guard
        .issue()
        .get_version_milestone_list(proj_id_or_key)
        .await?;
    Ok(versions)
}

pub async fn get_issues_by_milestone_name_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    project_id_or_key_str: String,
    milestone_name_str: String,
) -> Result<Vec<Issue>> {
    let client_guard = client.lock().await;

    let proj_id_or_key =
        ProjectIdOrKey::from_str(project_id_or_key_str.trim()).map_err(McpError::Core)?;

    let project_id_numeric = match proj_id_or_key.clone() {
        ProjectIdOrKey::Id(id) => id,
        ProjectIdOrKey::Key(key_val) => client_guard.project().get_project(key_val).await?.id,
        ProjectIdOrKey::EitherIdOrKey(id, _) => id,
    };

    let milestones = client_guard
        .issue()
        .get_version_milestone_list(proj_id_or_key.clone())
        .await?;

    let found_milestone_id = milestones
        .iter()
        .find(|m| m.name.to_lowercase() == milestone_name_str.to_lowercase())
        .map(|m| m.id);

    match found_milestone_id {
        Some(id) => {
            // Backlog constraints usually prevent duplicate milestone names within a project,
            // so finding the first match is generally safe.
            let params = GetIssueListParamsBuilder::default()
                .project_id(vec![project_id_numeric])
                .milestone_id(vec![id]) // Use the first found milestone ID
                .build()
                .map_err(|e| McpError::Parameter(e.to_string()))?; // Changed to Parameter error

            let issues = client_guard.issue().get_issue_list(params).await?;
            Ok(issues)
        }
        None => Err(McpError::MilestoneNotFoundByName {
            project: project_id_or_key_str,
            name: milestone_name_str,
        }),
    }
}
