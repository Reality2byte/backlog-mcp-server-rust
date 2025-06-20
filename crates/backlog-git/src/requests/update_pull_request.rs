use backlog_api_core::Error as ApiError;
use backlog_core::identifier::{Identifier, IssueId, UserId};
use derive_builder::Builder;

/// Parameters for updating a pull request.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdatePullRequestParams {
    /// The summary (title) of the pull request.
    #[builder(default)]
    pub summary: Option<String>,
    /// The description of the pull request.
    #[builder(default)]
    pub description: Option<String>,
    /// The ID of the related issue.
    #[builder(default)]
    pub issue_id: Option<IssueId>,
    /// The ID of the assignee user.
    #[builder(default)]
    pub assignee_id: Option<UserId>,
    /// List of user IDs to notify about this update.
    #[builder(default)]
    pub notified_user_ids: Option<Vec<UserId>>,
    /// Comment to add with the update.
    #[builder(default)]
    pub comment: Option<String>,
}

impl UpdatePullRequestParams {
    /// Creates a new instance with all optional fields set to None.
    pub fn new() -> Self {
        Self {
            summary: None,
            description: None,
            issue_id: None,
            assignee_id: None,
            notified_user_ids: None,
            comment: None,
        }
    }
}

impl Default for UpdatePullRequestParams {
    fn default() -> Self {
        Self::new()
    }
}

// Convert UpdatePullRequestParams to vector of pairs for form encoding
// because RequestBuilder.form() requires proper handling of array parameters like notifiedUserId[]
impl From<&UpdatePullRequestParams> for Vec<(String, String)> {
    fn from(params: &UpdatePullRequestParams) -> Self {
        let mut seq = Vec::new();

        // Add summary parameter
        if let Some(summary) = &params.summary {
            seq.push(("summary".to_string(), summary.clone()));
        }

        // Add description parameter
        if let Some(description) = &params.description {
            seq.push(("description".to_string(), description.clone()));
        }

        // Add issue ID parameter
        if let Some(issue_id) = params.issue_id {
            seq.push(("issueId".to_string(), issue_id.value().to_string()));
        }

        // Add assignee ID parameter
        if let Some(assignee_id) = params.assignee_id {
            seq.push(("assigneeId".to_string(), assignee_id.value().to_string()));
        }

        // Add notified user IDs as separate entries with "notifiedUserId[]" key
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids
                .iter()
                .for_each(|id| seq.push(("notifiedUserId[]".to_string(), id.value().to_string())));
        }

        // Add comment parameter
        if let Some(comment) = &params.comment {
            seq.push(("comment".to_string(), comment.clone()));
        }

        seq
    }
}
