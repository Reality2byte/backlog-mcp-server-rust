use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, IssueId, PullRequestNumber, UserId},
};
use derive_builder::Builder;

/// Parameters for updating a pull request.
///
/// This struct now includes all path information needed to construct the complete request.
#[cfg(feature = "writable")]
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdatePullRequestParams {
    /// The project ID or key where the repository is located.
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    pub repo_id_or_name: RepositoryIdOrName,
    /// The pull request number.
    pub pr_number: PullRequestNumber,
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

#[cfg(feature = "writable")]
impl UpdatePullRequestParams {
    /// Creates a new instance with path parameters and all optional fields set to None.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            pr_number,
            summary: None,
            description: None,
            issue_id: None,
            assignee_id: None,
            notified_user_ids: None,
            comment: None,
        }
    }
}

/// Convert UpdatePullRequestParams to form-encoded parameters for HTTP requests.
/// Handles array parameters with [] notation as required by Backlog API.
#[cfg(feature = "writable")]
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

#[cfg(feature = "writable")]
impl IntoRequest for UpdatePullRequestParams {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
            self.project_id_or_key,
            self.repo_id_or_name,
            self.pr_number.value()
        )
    }

    fn to_form(&self) -> Vec<(String, String)> {
        From::from(self)
    }
}

