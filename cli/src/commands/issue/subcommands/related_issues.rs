//! Related issue operations

use crate::commands::common::CliResult;
use anyhow::Context;
use backlog_api_client::IssueIdOrKey;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::Identifier;
use backlog_issue::{GetRelatedIssuesParams, RelatedIssue};

fn parse_issue(issue_id_or_key: &str) -> CliResult<IssueIdOrKey> {
    issue_id_or_key
        .parse()
        .with_context(|| format!("Failed to parse issue_id_or_key '{issue_id_or_key}'"))
}

/// Resolve issue key to ID
#[cfg(feature = "issue_writable")]
async fn resolve_issue_id(
    client: &BacklogApiClient,
    issue_id_or_key: &str,
) -> CliResult<backlog_core::identifier::IssueId> {
    match parse_issue(issue_id_or_key)? {
        IssueIdOrKey::Id(id) => Ok(id),
        key => Ok(client
            .issue()
            .get_issue(backlog_issue::GetIssueParams::new(key))
            .await?
            .id),
    }
}

fn print_related_issue(index: usize, related: &RelatedIssue) {
    let issue = &related.issue;
    println!("{}. [{}] {}", index, issue.issue_key, issue.summary);
    println!("   ID: {}", issue.id.value());
    println!("   Status: {}", issue.status.name);
    if let Some(assignee) = &issue.assignee {
        println!("   Assignee: {}", assignee.name);
    }
    println!("   Relation: {}", related.relation_type);
}

/// List related issues
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/relatedIssues`
pub async fn list_related_issues(
    client: &BacklogApiClient,
    issue_id_or_key: String,
) -> CliResult<()> {
    let parsed = parse_issue(&issue_id_or_key)?;
    let related = client
        .issue()
        .get_related_issues(GetRelatedIssuesParams::new(parsed))
        .await?;

    if related.is_empty() {
        println!("No related issues found for this issue.");
        return Ok(());
    }
    println!("Found {} related issue(s):", related.len());
    for (index, item) in related.iter().enumerate() {
        print_related_issue(index + 1, item);
    }
    Ok(())
}

/// Add a related issue
///
/// Corresponds to `POST /api/v2/issues/:issueIdOrKey/relatedIssues`
#[cfg(feature = "issue_writable")]
pub async fn add_related_issue(
    client: &BacklogApiClient,
    issue_id_or_key: String,
    target_issue_id_or_key: String,
) -> CliResult<()> {
    use backlog_issue::AddRelatedIssueParams;

    let parsed = parse_issue(&issue_id_or_key)?;
    let target_id = resolve_issue_id(client, &target_issue_id_or_key).await?;
    let related = client
        .issue()
        .add_related_issue(AddRelatedIssueParams::new(parsed, target_id))
        .await?;

    println!("✅ Related issue added to {issue_id_or_key}:");
    print_related_issue(1, &related);
    Ok(())
}

/// Remove a related issue
///
/// Corresponds to `DELETE /api/v2/issues/:issueIdOrKey/relatedIssues/:relatedIssueId`
#[cfg(feature = "issue_writable")]
pub async fn remove_related_issue(
    client: &BacklogApiClient,
    issue_id_or_key: String,
    related_issue_id_or_key: String,
) -> CliResult<()> {
    use backlog_issue::DeleteRelatedIssueParams;

    let parsed = parse_issue(&issue_id_or_key)?;
    let related_id = resolve_issue_id(client, &related_issue_id_or_key).await?;
    let related = client
        .issue()
        .delete_related_issue(DeleteRelatedIssueParams::new(parsed, related_id))
        .await?;

    println!("✅ Related issue removed from {issue_id_or_key}:");
    print_related_issue(1, &related);
    Ok(())
}
