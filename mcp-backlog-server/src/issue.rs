use backlog_api_client::client::BacklogApiClient;
use backlog_core::IssueKey;
use backlog_issue::Issue;
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

    let issue = client
        .issue()
        .get_issue(issue_key.clone())
        .await?;

    Ok(issue)
}
