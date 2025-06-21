use backlog_api_core::{Error as ApiError, IntoRequest, Result};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, PullRequestCommentId, PullRequestNumber},
};
use derive_builder::Builder;
use reqwest::Client as ReqwestClient;
use url::Url;

/// Parameters for updating a pull request comment.
///
/// This struct now includes all path information needed to construct the complete request.
#[cfg(feature = "writable")]
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdatePullRequestCommentParams {
    /// The project ID or key where the repository is located.
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    pub repo_id_or_name: RepositoryIdOrName,
    /// The pull request number.
    pub pr_number: PullRequestNumber,
    /// The comment ID to update.
    pub comment_id: PullRequestCommentId,
    /// The updated content of the comment.
    pub content: String,
}

#[cfg(feature = "writable")]
impl UpdatePullRequestCommentParams {
    /// Creates a new instance with all required fields.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
        comment_id: PullRequestCommentId,
        content: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            pr_number,
            comment_id,
            content: content.into(),
        }
    }
}

/// Convert UpdatePullRequestCommentParams to form-encoded parameters for HTTP requests.
#[cfg(feature = "writable")]
impl From<&UpdatePullRequestCommentParams> for Vec<(String, String)> {
    fn from(params: &UpdatePullRequestCommentParams) -> Self {
        vec![("content".to_string(), params.content.clone())]
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdatePullRequestCommentParams {
    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/{}",
            self.project_id_or_key,
            self.repo_id_or_name,
            self.pr_number.value(),
            self.comment_id.value()
        );
        let form: Vec<(String, String)> = (&self).into();
        self.patch(client, base_url, path, &form)
    }
}
