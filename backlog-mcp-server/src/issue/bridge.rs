use super::request::{
    AddCommentRequest, DownloadAttachmentRequest, GetAttachmentListRequest,
    GetIssueCommentsRequest, GetIssueDetailsRequest, GetIssueSharedFilesRequest,
    GetIssuesByMilestoneNameRequest, GetVersionMilestoneListRequest, UpdateIssueRequest,
};
use crate::error::{Error as McpError, Result};
use crate::util::{MatchResult, find_by_name_from_array};
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    AddCommentParams, Attachment, AttachmentId, Comment, DownloadedFile, GetCommentListParams,
    GetIssueListParamsBuilder, Issue, IssueIdOrKey, IssueKey, IssueSharedFile, Milestone,
    ProjectIdOrKey, UpdateIssueParams, backlog_issue, backlog_project,
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) async fn get_issue_details(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssueDetailsRequest,
) -> Result<Issue> {
    let client_guard = client.lock().await;
    let parsed_issue_key = IssueKey::from_str(req.issue_key.trim())?;
    let issue = client_guard
        .issue()
        .get_issue(backlog_issue::GetIssueParams::new(parsed_issue_key.clone()))
        .await?;
    Ok(issue)
}

pub(crate) async fn get_version_milestone_list(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetVersionMilestoneListRequest,
) -> Result<Vec<Milestone>> {
    let client_guard = client.lock().await;
    let proj_id_or_key = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;
    let versions = client_guard
        .project()
        .get_version_milestone_list(backlog_project::GetVersionMilestoneListParams::new(
            proj_id_or_key,
        ))
        .await?;
    Ok(versions)
}

pub(crate) async fn get_issues_by_milestone_name(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssuesByMilestoneNameRequest,
) -> Result<Vec<Issue>> {
    let proj_id_or_key = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;

    let client_guard = client.lock().await;

    let all_project_milestones = client_guard
        .project()
        .get_version_milestone_list(backlog_project::GetVersionMilestoneListParams::new(
            proj_id_or_key.clone(),
        ))
        .await?;

    let milestone =
        find_milestone_by_name(&all_project_milestones, &req.milestone_name, proj_id_or_key)?;
    let params = GetIssueListParamsBuilder::default()
        .project_id(vec![milestone.project_id])
        .milestone_id(vec![milestone.id])
        .build()?;

    let issues = client_guard.issue().get_issue_list(params).await?;
    Ok(issues)
}

fn find_milestone_by_name(
    milestones: &[Milestone],
    milestone_name: &str,
    project_id_or_key: ProjectIdOrKey,
) -> Result<Milestone> {
    match find_by_name_from_array(milestones, milestone_name, |m| &m.name) {
        MatchResult::Exact(milestone) => Ok(milestone),
        MatchResult::Suggestion(suggestions) => Err(McpError::MilestoneNotFoundByName {
            project_id_or_key,
            original_name: milestone_name.to_string(),
            suggestions: Some(suggestions),
        }),
        MatchResult::None => Err(McpError::MilestoneNotFoundByName {
            project_id_or_key,
            original_name: milestone_name.to_string(),
            suggestions: None,
        }),
    }
}

#[cfg(feature = "issue_writable")]
pub(crate) async fn update_issue_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: UpdateIssueRequest,
) -> Result<Issue> {
    if req.summary.is_none() && req.description.is_none() {
        return Err(McpError::NothingToUpdate);
    }

    let update_params = UpdateIssueParams::try_from(req)?;

    let client_guard = client.lock().await;
    let updated_issue = client_guard.issue().update_issue(update_params).await?;
    Ok(updated_issue)
}

pub(crate) async fn get_issue_comments_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssueCommentsRequest,
) -> Result<Vec<Comment>> {
    let comment_params = GetCommentListParams::try_from(req)?;

    let client_guard = client.lock().await;
    let comments = client_guard
        .issue()
        .get_comment_list(comment_params)
        .await?;
    Ok(comments)
}

pub(crate) async fn get_attachment_list_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetAttachmentListRequest,
) -> Result<Vec<Attachment>> {
    let parsed_issue_id_or_key =
        IssueIdOrKey::from_str(req.issue_id_or_key.trim()).map_err(|e| {
            McpError::Parameter(format!(
                "Invalid issueIdOrKey: {}. Error: {}",
                req.issue_id_or_key, e
            ))
        })?;

    let client_guard = client.lock().await;
    let attachments = client_guard
        .issue()
        .get_attachment_list(backlog_issue::GetAttachmentListParams::new(
            parsed_issue_id_or_key.clone(),
        ))
        .await?;
    Ok(attachments)
}

pub(crate) async fn download_issue_attachment_file(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DownloadAttachmentRequest,
) -> Result<DownloadedFile> {
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(&req.issue_id_or_key)?;
    let parsed_attachment_id = AttachmentId::new(req.attachment_id);

    let client_guard = client.lock().await;
    let params = backlog_issue::requests::GetAttachmentFileParams::new(
        parsed_issue_id_or_key,
        parsed_attachment_id,
    );
    let attachment = client_guard.issue().get_attachment_file(params).await?;
    Ok(attachment)
}

#[cfg(feature = "issue_writable")]
pub(crate) async fn add_comment_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: AddCommentRequest,
) -> Result<Comment> {
    let add_comment_params = AddCommentParams::try_from(req)?;

    let client_guard = client.lock().await;
    let comment = client_guard.issue().add_comment(add_comment_params).await?;
    Ok(comment)
}

pub(crate) async fn get_issue_shared_files_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssueSharedFilesRequest,
) -> Result<Vec<IssueSharedFile>> {
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;

    let client_guard = client.lock().await;
    let shared_files = client_guard
        .issue()
        .get_shared_file_list(backlog_issue::GetSharedFileListParams::new(
            parsed_issue_id_or_key,
        ))
        .await?;
    Ok(shared_files)
}
