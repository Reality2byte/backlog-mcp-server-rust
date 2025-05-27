use backlog_api_client::client::BacklogApiClient;
use backlog_core::{IssueKey, ProjectIdOrKey};
use backlog_issue::{Issue, Milestone};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::Result;

pub async fn get_issue_details(
    client: Arc<Mutex<BacklogApiClient>>,
    issue_key: String,
) -> Result<Issue> {
    let client = client.lock().await;
    let issue_key = IssueKey::from_str(issue_key.trim())?;
    let issue = client.issue().get_issue(issue_key.clone()).await?;
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
