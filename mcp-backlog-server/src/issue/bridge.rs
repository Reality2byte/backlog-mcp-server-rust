use super::request::{
    GetIssueCommentsRequest, // Added
    GetIssueDetailsRequest,
    GetIssuesByMilestoneNameRequest,
    GetVersionMilestoneListRequest,
    UpdateIssueRequest,
};
use crate::error::{Error as McpError, Result};
use crate::util::{MatchResult, find_by_name_from_array};
use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    Comment, // Added
    // Corrected paths for these comment-related types:
    GetCommentListParamsBuilder,
    GetIssueListParamsBuilder,
    Issue,
    IssueIdOrKey,
    IssueKey,
    Milestone,
    ProjectIdOrKey,
    UpdateIssueParamsBuilder,
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
        .get_issue(parsed_issue_key.clone())
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
        .issue()
        .get_version_milestone_list(proj_id_or_key)
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
        .issue()
        .get_version_milestone_list(proj_id_or_key.clone())
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

    let issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
    let update_params = UpdateIssueParamsBuilder::from(req).build()?;

    let client_guard = client.lock().await;
    let updated_issue = client_guard
        .issue()
        .update_issue(issue_id_or_key, &update_params)
        .await?;
    Ok(updated_issue)
}

pub(crate) async fn get_issue_comments_impl(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetIssueCommentsRequest,
) -> Result<Vec<Comment>> {
    let parsed_issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
    let comment_params = GetCommentListParamsBuilder::try_from(req)?.build()?;

    let client_guard = client.lock().await;
    let comments = client_guard
        .issue()
        .get_comment_list(parsed_issue_id_or_key, Some(comment_params))
        .await?;

    Ok(comments)
}
