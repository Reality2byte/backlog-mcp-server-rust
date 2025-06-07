use crate::error::{Error, Result}; // Added Error
use crate::git::request::{
    DownloadPullRequestAttachmentRequest, GetPullRequestAttachmentListRequest,
}; // Added DownloadPullRequestAttachmentRequest
use backlog_api_client::bytes::Bytes; // Added Bytes
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    AttachmentId, // Added AttachmentId
    PrNumber,     // Added PrNumber
    ProjectIdOrKey,
    PullRequest,
    PullRequestAttachment,
    Repository,
    RepositoryIdOrName, // GitPullRequestAttachment を PullRequestAttachment に変更
};
use backlog_core::Identifier; // Added for .value()
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
        .get_pull_request(proj_id_or_key, repo_id_or_name, PrNumber::from(pr_number))
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
        .get_pull_request_attachment_list(
            &project_id_or_key,
            &repo_id_or_name,
            PrNumber::from(req.pr_number),
        )
        .await?)
}

pub(crate) async fn download_pr_attachment_bridge(
    req: DownloadPullRequestAttachmentRequest,
    client: Arc<Mutex<BacklogApiClient>>,
) -> Result<(String, Bytes)> {
    let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
    let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
    let pr_number = PrNumber::from(req.pr_number); // Changed to PrNumber
    let target_attachment_id_val = req.attachment_id; // This is u32

    let client_guard = client.lock().await;

    // 1. Get the list of attachments to find the filename
    // Pass references to project_id_or_key and repo_id_or_name
    let attachments = client_guard
        .git()
        .get_pull_request_attachment_list(&project_id_or_key, &repo_id_or_name, pr_number)
        .await?;

    let id_to_find_rhs = target_attachment_id_val as u64;
    let target_attachment = attachments.iter().find(|att| {
        let current_att_id_lhs: u64 = att.id.value() as u64; // Cast u32 (compiler inferred) to u64
        current_att_id_lhs == id_to_find_rhs
    });

    match target_attachment {
        Some(attachment_info) => {
            let filename = attachment_info.name.clone();
            let attachment_id_for_download = AttachmentId::new(target_attachment_id_val); // Create AttachmentId for download

            // 2. Download the actual file content
            // Now we can move project_id_or_key and repo_id_or_name as they were not moved before
            let file_bytes = client_guard
                .git()
                .download_pull_request_attachment(
                    project_id_or_key,
                    repo_id_or_name,
                    pr_number,
                    attachment_id_for_download,
                )
                .await?;
            Ok((filename, file_bytes))
        }
        None => Err(Error::PullRequestAttachmentNotFound {
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            attachment_id: target_attachment_id_val,
        }),
    }
}
