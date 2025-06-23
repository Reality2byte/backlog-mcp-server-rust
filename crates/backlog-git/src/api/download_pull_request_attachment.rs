use backlog_api_core::{HttpMethod, IntoDownloadRequest, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{PullRequestAttachmentId, PullRequestNumber},
};
use client::DownloadedFile;

pub type DownloadPullRequestAttachmentResponse = DownloadedFile;

#[derive(Debug, Clone)]
pub struct DownloadPullRequestAttachmentParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
    pub number: PullRequestNumber,
    pub attachment_id: PullRequestAttachmentId,
}

impl DownloadPullRequestAttachmentParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        number: impl Into<PullRequestNumber>,
        attachment_id: impl Into<PullRequestAttachmentId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            number: number.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl IntoRequest for DownloadPullRequestAttachmentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
            self.project_id_or_key, self.repo_id_or_name, self.number, self.attachment_id
        )
    }
}

impl IntoDownloadRequest for DownloadPullRequestAttachmentParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
            self.project_id_or_key, self.repo_id_or_name, self.number, self.attachment_id
        )
    }
}
