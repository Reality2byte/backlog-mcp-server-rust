use crate::error::{Error, Result};
use crate::git::request::{
    DownloadPullRequestAttachmentRequest, GetPullRequestAttachmentListRequest,
    GetPullRequestCommentListRequest, GetPullRequestDetailsRequest, GetRepositoryDetailsRequest,
    GetRepositoryListRequest, ListPullRequestsRequest,
};
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    DownloadedFile, GetPullRequestCommentListParams, ProjectIdOrKey, PullRequest,
    PullRequestAttachment, PullRequestAttachmentId, PullRequestComment, PullRequestNumber,
    Repository, RepositoryIdOrName,
};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

/// Helper function to implement the get_repository_list tool.
pub(crate) async fn get_repository_list(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetRepositoryListRequest,
) -> Result<Vec<Repository>> {
    let project_id = req.project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;
    let repositories = client_guard.git().get_repository_list(project_id).await?;
    Ok(repositories)
}

pub(crate) async fn get_repository(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetRepositoryDetailsRequest,
) -> Result<Repository> {
    let proj_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;

    let client_guard = client.lock().await;
    let repository = client_guard
        .git()
        .get_repository(proj_id_or_key, repo_id_or_name)
        .await?;
    Ok(repository)
}

pub(crate) async fn get_pull_request_list(
    client: Arc<Mutex<BacklogApiClient>>,
    req: ListPullRequestsRequest,
) -> Result<Vec<PullRequest>> {
    let proj_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;

    let client_guard = client.lock().await;
    let pull_requests = client_guard
        .git()
        .get_pull_request_list(proj_id_or_key, repo_id_or_name)
        .await?;
    Ok(pull_requests)
}

pub(crate) async fn get_pull_request(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetPullRequestDetailsRequest,
) -> Result<PullRequest> {
    let proj_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
    let pr_number = PullRequestNumber::from(req.pr_number);

    let client_guard = client.lock().await;
    let pull_request = client_guard
        .git()
        .get_pull_request(proj_id_or_key, repo_id_or_name, pr_number)
        .await?;
    Ok(pull_request)
}

pub(crate) async fn get_pull_request_attachment_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetPullRequestAttachmentListRequest,
) -> Result<Vec<PullRequestAttachment>> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;

    let client_guard = client.lock().await;
    Ok(client_guard
        .git()
        .get_pull_request_attachment_list(
            &project_id_or_key,
            &repo_id_or_name,
            PullRequestNumber::from(req.pr_number),
        )
        .await?)
}

pub(crate) async fn download_pr_attachment_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DownloadPullRequestAttachmentRequest,
) -> Result<DownloadedFile> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
    let pr_number = PullRequestNumber::from(req.pr_number);
    let attachment_id_for_download = PullRequestAttachmentId::new(req.attachment_id);

    let client_guard = client.lock().await;

    // The download_pull_request_attachment method in backlog-git now returns (filename, content_type, bytes)
    // due to changes in client.download_file_raw.
    client_guard
        .git()
        .download_pull_request_attachment(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            attachment_id_for_download,
        )
        .await
        .map_err(Error::from)
}

pub(crate) async fn get_pull_request_comment_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetPullRequestCommentListRequest,
) -> Result<Vec<PullRequestComment>> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
    let pr_number = PullRequestNumber::from(req.pr_number);

    let params = GetPullRequestCommentListParams::try_from(req)?;

    let client_guard = client.lock().await;
    Ok(client_guard
        .git()
        .get_pull_request_comment_list(project_id_or_key, repo_id_or_name, pr_number, params)
        .await?)
}
