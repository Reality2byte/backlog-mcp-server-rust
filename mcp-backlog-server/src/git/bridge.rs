use crate::error::Result;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{ProjectIdOrKey, PullRequest, Repository, RepositoryIdOrName};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

/// Helper function to implement the get_repository_list tool.
pub async fn get_repository_list_impl(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    project_id_or_key: String,
) -> Result<Vec<Repository>> {
    let project_id = project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;
    let repositories = client_guard.git().list_repositories(project_id).await?;
    Ok(repositories)
}

pub async fn get_repository_details_impl(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    project_id_or_key: String,
    repo_id_or_name: String,
) -> Result<Repository> {
    let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(repo_id_or_name.trim())?;

    let client_guard = client.lock().await; // Added lock

    let repository = client_guard
        .git()
        .get_repository(proj_id_or_key, repo_id_or_name)
        .await?;
    Ok(repository)
}

pub async fn list_pull_requests_impl(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    project_id_or_key: String,
    repo_id_or_name: String,
) -> Result<Vec<PullRequest>> {
    let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(repo_id_or_name.trim())?;

    let client_guard = client.lock().await; // Added lock
    let pull_requests = client_guard
        .git()
        .list_pull_requests(proj_id_or_key, repo_id_or_name)
        .await?;
    Ok(pull_requests)
}

pub async fn get_pull_request_details_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    project_id_or_key: String,
    repo_id_or_name: String,
    pr_number: u64,
) -> Result<PullRequest> {
    let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;

    let repo_id_or_name = RepositoryIdOrName::from_str(repo_id_or_name.trim())?;

    let client_guard = client.lock().await; // Added lock
    let pull_request = client_guard
        .git()
        .get_pull_request(proj_id_or_key, repo_id_or_name, pr_number)
        .await?;
    Ok(pull_request)
}
