use backlog_api_client::client::BacklogApiClient;
use backlog_core::{IssueIdOrKey, IssueKey, ProjectIdOrKey};
use backlog_issue::{
    Issue, Milestone,
    requests::{GetIssueListParamsBuilder, UpdateIssueParamsBuilder},
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::{Error as McpError, Result};
use crate::util::{MatchResult, find_by_name_from_array};

pub async fn get_issue_details(
    client: Arc<Mutex<BacklogApiClient>>,
    issue_key: String,
) -> Result<Issue> {
    let client_guard = client.lock().await;
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
    let proj_id_or_key =
        ProjectIdOrKey::from_str(project_id_or_key_str.trim()).map_err(McpError::Core)?;
    let versions = client_guard
        .issue()
        .get_version_milestone_list(proj_id_or_key)
        .await?;
    Ok(versions)
}

pub async fn get_issues_by_milestone_name_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    project_id_or_key_str: String,
    milestone_name: String,
) -> Result<Vec<Issue>> {
    let proj_id_or_key =
        ProjectIdOrKey::from_str(project_id_or_key_str.trim()).map_err(McpError::Core)?;

    let client_guard = client.lock().await;

    let all_project_milestones = client_guard
        .issue()
        .get_version_milestone_list(proj_id_or_key.clone())
        .await?;

    let result =
        find_by_name_from_array(&all_project_milestones, &milestone_name, |m| {
            &m.name
        });
    match result {
        MatchResult::Exact(milestone) => {
            let params = GetIssueListParamsBuilder::default()
                .project_id(vec![milestone.project_id])
                .milestone_id(vec![milestone.id])
                .build()
                .map_err(|e| McpError::Parameter(e.to_string()))?;
            let issues = client_guard.issue().get_issue_list(params).await?;
            Ok(issues)
        }
        MatchResult::Suggestion(suggestions) => Err(McpError::MilestoneNotFoundByName {
            project: project_id_or_key_str,
            original_name: milestone_name,
            suggestions: Some(suggestions),
        }),
        MatchResult::None => Err(McpError::MilestoneNotFoundByName {
            project: project_id_or_key_str,
            original_name: milestone_name,
            suggestions: None,
        }),
    }
}

#[cfg(feature = "issue_writable")]
pub async fn update_issue_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    issue_id_or_key_str: String,
    summary: Option<String>,
    description: Option<String>,
) -> Result<backlog_issue::Issue> {
    if summary.is_none() && description.is_none() {
        return Err(McpError::NothingToUpdate);
    }

    let client_guard = client.lock().await;

    let issue_id_or_key =
        IssueIdOrKey::from_str(issue_id_or_key_str.trim()).map_err(McpError::Core)?;

    let mut params_builder = UpdateIssueParamsBuilder::default();
    if let Some(s) = summary {
        params_builder.summary(s);
    }
    if let Some(d) = description {
        params_builder.description(d);
    }
    let update_params = params_builder
        .build()
        .map_err(|e| McpError::Parameter(e.to_string()))?;

    let updated_issue = client_guard
        .issue()
        .update_issue(issue_id_or_key, &update_params)
        .await?;
    Ok(updated_issue)
}
