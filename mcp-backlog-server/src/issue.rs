use backlog_api_client::client::BacklogApiClient;
use backlog_core::{IssueIdOrKey, IssueKey, ProjectIdOrKey};
use backlog_issue::{
    Issue, Milestone,
    requests::{GetIssueListParamsBuilder, UpdateIssueParamsBuilder},
};
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::{Error as McpError, Result};

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

    let all_project_milestones = client_guard
        .issue()
        .get_version_milestone_list(proj_id_or_key.clone())
        .await?;

    let result =
        find_by_name_from_array(&all_project_milestones, &milestone_name_user_input, |m| {
            &m.name
        });
    match result {
        MatchResult::Exact(milestone) => {
            let params = GetIssueListParamsBuilder::default()
                .project_id(vec![project_id_numeric])
                .milestone_id(vec![milestone.id])
                .build()
                .map_err(|e| McpError::Parameter(e.to_string()))?;
            let issues = client_guard.issue().get_issue_list(params).await?;
            Ok(issues)
        }
        MatchResult::Suggestion(suggestions) => Err(McpError::MilestoneNotFoundByName {
            project: project_id_or_key_str,
            original_name: milestone_name_user_input,
            suggestions: Some(suggestions),
        }),
        MatchResult::None => Err(McpError::MilestoneNotFoundByName {
            project: project_id_or_key_str,
            original_name: milestone_name_user_input,
            suggestions: None,
        }),
    }
}

enum MatchResult<T> {
    Exact(T),
    Suggestion(Vec<String>),
    None,
}

fn find_by_name_from_array<T: Clone>(
    array: &[T],
    name: &str,
    name_getter: impl Fn(&T) -> &String,
) -> MatchResult<T> {
    let preprocessed_name = |name: &str| name.to_lowercase().replace([' ', '　'], "");

    let name = preprocessed_name(name);

    for m in array {
        if preprocessed_name(name_getter(m)) == name {
            return MatchResult::Exact(m.clone());
        }
    }

    let get_prefix_length = |s1: &str, s2: &str| {
        s1.chars()
            .zip(s2.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .count()
    };

    let mut candidates = Vec::new();
    for m in array {
        let dist = strsim::levenshtein(&name, &preprocessed_name(name_getter(m)));
        if dist <= 2 {
            let prefix_len = get_prefix_length(&name, name_getter(m));
            candidates.push((name_getter(m), dist, prefix_len));
        }
    }

    if candidates.is_empty() {
        MatchResult::None
    } else {
        candidates.sort_by(|a, b| match a.1.cmp(&b.1) {
            Ordering::Equal => b.2.cmp(&a.2),
            other => other,
        });
        MatchResult::Suggestion(
            candidates
                .into_iter()
                .map(|(name, _, _)| name.clone())
                .collect(),
        )
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
