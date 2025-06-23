use crate::models::PullRequestAttachment;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{PullRequestAttachmentId, PullRequestNumber},
};

pub type DeletePullRequestAttachmentResponse = PullRequestAttachment;

#[derive(Debug, Clone)]
pub struct DeletePullRequestAttachmentParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
    pub number: PullRequestNumber,
    pub attachment_id: PullRequestAttachmentId,
}

impl DeletePullRequestAttachmentParams {
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

impl IntoRequest for DeletePullRequestAttachmentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
            self.project_id_or_key, self.repo_id_or_name, self.number, self.attachment_id
        )
    }
}
