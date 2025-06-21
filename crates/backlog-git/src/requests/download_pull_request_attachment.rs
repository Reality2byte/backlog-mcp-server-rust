use backlog_api_core::{Error as ApiError, GetRequest, IntoRequest, Result};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, PullRequestAttachmentId, PullRequestNumber},
};
use derive_builder::Builder;
use reqwest::Client as ReqwestClient;
use url::Url;

/// Parameters for downloading a pull request attachment.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct DownloadPullRequestAttachmentParams {
    /// The project ID or key.
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    pub repo_id_or_name: RepositoryIdOrName,
    /// The pull request number.
    pub pr_number: PullRequestNumber,
    /// The attachment ID.
    pub attachment_id: PullRequestAttachmentId,
}

impl DownloadPullRequestAttachmentParams {
    /// Creates a new instance with the required parameters.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
        attachment_id: PullRequestAttachmentId,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            pr_number,
            attachment_id,
        }
    }
}

impl IntoRequest for DownloadPullRequestAttachmentParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
            self.project_id_or_key,
            self.repo_id_or_name,
            self.pr_number.value(),
            self.attachment_id.value()
        )
    }

    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        self.get(client, base_url)
    }
}

impl GetRequest for DownloadPullRequestAttachmentParams {}
