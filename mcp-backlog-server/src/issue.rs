use backlog_api_client::client::BacklogApiClient;
use backlog_core::{IssueKey, ProjectIdOrKey, ProjectKey, identifier::MilestoneId}; // Added MilestoneId
use backlog_issue::{Issue, Milestone, requests::GetIssueListParamsBuilder};
use std::cmp::Ordering; // Added Ordering
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::{Error as McpError, Result};

// Helper function to preprocess milestone names for comparison
fn preprocess_milestone_name(name: &str) -> String {
    name.to_lowercase().replace([' ', '　'], "")
}

// Helper function to calculate the length of the common prefix of two strings
fn common_prefix_len(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .count()
}

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
    milestone_name_user_input: String, // Corrected argument name
) -> Result<Vec<Issue>> {
    let client_guard = client.lock().await;

    let proj_id_or_key =
        ProjectIdOrKey::from_str(project_id_or_key_str.trim()).map_err(McpError::Core)?;

    let project_id_numeric = match proj_id_or_key.clone() {
        ProjectIdOrKey::Id(id) => id,
        ProjectIdOrKey::Key(key_val) => client_guard.project().get_project(key_val).await?.id,
        ProjectIdOrKey::EitherIdOrKey(id, _) => id,
    };

    let all_project_milestones = client_guard // Renamed variable for clarity
        .issue()
        .get_version_milestone_list(proj_id_or_key.clone())
        .await?;

    let preprocessed_user_input = preprocess_milestone_name(&milestone_name_user_input);

    // Step 1: Try to find an exact match after preprocessing
    let mut exactly_matched_milestone_id: Option<MilestoneId> = None;
    for m in &all_project_milestones {
        // Use renamed variable
        if preprocess_milestone_name(&m.name) == preprocessed_user_input {
            exactly_matched_milestone_id = Some(m.id);
            break;
        }
    }

    if let Some(id_to_use) = exactly_matched_milestone_id {
        let params = GetIssueListParamsBuilder::default()
            .project_id(vec![project_id_numeric])
            .milestone_id(vec![id_to_use])
            .build()
            .map_err(|e| McpError::Parameter(e.to_string()))?;
        let issues = client_guard.issue().get_issue_list(params).await?;
        return Ok(issues);
    }

    // Step 2: If no exact match, find suggestions using Levenshtein distance
    let mut levenshtein_candidates = Vec::new();
    for m in &all_project_milestones {
        // Use renamed variable
        let dist = strsim::levenshtein(
            &preprocessed_user_input,
            &preprocess_milestone_name(&m.name),
        );
        if dist <= 2 {
            let prefix_len = common_prefix_len(&milestone_name_user_input, &m.name);
            levenshtein_candidates.push((m.name.clone(), dist, prefix_len));
        }
    }

    let suggestions: Option<Vec<String>> = if levenshtein_candidates.is_empty() {
        None
    } else {
        levenshtein_candidates.sort_by(|a, b| match a.1.cmp(&b.1) {
            Ordering::Equal => b.2.cmp(&a.2),
            other => other,
        });
        Some(
            levenshtein_candidates
                .into_iter()
                .map(|(name, _, _)| name)
                .collect(),
        )
    };

    Err(McpError::MilestoneNotFoundByName {
        project: project_id_or_key_str,
        original_name: milestone_name_user_input,
        suggestions,
    })
}
