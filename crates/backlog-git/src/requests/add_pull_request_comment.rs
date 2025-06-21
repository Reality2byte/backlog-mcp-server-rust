use backlog_api_core::{Error as ApiError, IntoRequest, PostRequest, Result};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, PullRequestNumber, UserId},
};
use derive_builder::Builder;
use reqwest::Client as ReqwestClient;
use url::Url;

/// Parameters for adding a comment to a pull request.
///
/// This struct now includes all path information needed to construct the complete request.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct AddPullRequestCommentParams {
    /// The project ID or key where the repository is located.
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    pub repo_id_or_name: RepositoryIdOrName,
    /// The pull request number.
    pub pr_number: PullRequestNumber,
    /// The content of the comment.
    pub content: String,
    /// List of user IDs to notify about this comment.
    #[builder(default)]
    pub notified_user_ids: Option<Vec<UserId>>,
}

impl AddPullRequestCommentParams {
    /// Creates a new instance with all required fields.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
        content: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            pr_number,
            content: content.into(),
            notified_user_ids: None,
        }
    }
}

// Convert AddPullRequestCommentParams to vector of pairs for form encoding
// because RequestBuilder.form() requires proper handling of array parameters like notifiedUserId[]
impl From<&AddPullRequestCommentParams> for Vec<(String, String)> {
    fn from(params: &AddPullRequestCommentParams) -> Self {
        let mut seq = Vec::new();

        // Add content parameter
        seq.push(("content".to_string(), params.content.clone()));

        // Add notified user IDs as separate entries with "notifiedUserId[]" key
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids
                .iter()
                .for_each(|id| seq.push(("notifiedUserId[]".to_string(), id.value().to_string())));
        }

        seq
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddPullRequestCommentParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
            self.project_id_or_key,
            self.repo_id_or_name,
            self.pr_number.value()
        )
    }

    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        self.post(client, base_url)
    }
}

#[cfg(feature = "writable")]
impl PostRequest for AddPullRequestCommentParams {
    fn to_form(&self) -> Vec<(String, String)> {
        From::from(self)
    }
}
