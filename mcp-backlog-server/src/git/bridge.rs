use crate::error::Result;
use crate::git::request::GetPullRequestAttachmentListRequest;
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    ProjectIdOrKey,
    PullRequest,
    PullRequestAttachment,
    Repository,
    RepositoryIdOrName, // GitPullRequestAttachment を PullRequestAttachment に変更
};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

/// Helper function to implement the get_repository_list tool.
pub(crate) async fn get_repository_list(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    project_id_or_key: String,
) -> Result<Vec<Repository>> {
    let project_id = project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;
    let repositories = client_guard.git().get_repository_list(project_id).await?;
    Ok(repositories)
}

pub(crate) async fn get_repository(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    project_id_or_key: String,
    repo_id_or_name: String,
) -> Result<Repository> {
    let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(repo_id_or_name.trim())?;

    let client_guard = client.lock().await;
    let repository = client_guard
        .git()
        .get_repository(proj_id_or_key, repo_id_or_name)
        .await?;
    Ok(repository)
}

pub(crate) async fn get_pull_request_list(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    project_id_or_key: String,
    repo_id_or_name: String,
) -> Result<Vec<PullRequest>> {
    let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(repo_id_or_name.trim())?;

    let client_guard = client.lock().await;
    let pull_requests = client_guard
        .git()
        .get_pull_request_list(proj_id_or_key, repo_id_or_name)
        .await?;
    Ok(pull_requests)
}

pub(crate) async fn get_pull_request(
    client: Arc<Mutex<BacklogApiClient>>,
    project_id_or_key: String,
    repo_id_or_name: String,
    pr_number: u64,
) -> Result<PullRequest> {
    let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(repo_id_or_name.trim())?;

    let client_guard = client.lock().await;
    let pull_request = client_guard
        .git()
        .get_pull_request(proj_id_or_key, repo_id_or_name, pr_number)
        .await?;
    Ok(pull_request)
}

pub(crate) async fn get_pull_request_attachment_list_tool(
    req: GetPullRequestAttachmentListRequest,
    client: Arc<Mutex<BacklogApiClient>>,
) -> Result<Vec<PullRequestAttachment>> {
    // GitPullRequestAttachment を PullRequestAttachment に変更
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;

    let client_guard = client.lock().await;
    Ok(client_guard
        .git()
        .get_pull_request_attachment_list(&project_id_or_key, &repo_id_or_name, req.pr_number)
        .await?)
}
