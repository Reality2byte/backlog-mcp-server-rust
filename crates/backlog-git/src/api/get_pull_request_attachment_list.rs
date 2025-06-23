use crate::models::PullRequestAttachment;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName, identifier::PullRequestNumber};

pub type GetPullRequestAttachmentListResponse = Vec<PullRequestAttachment>;

#[derive(Debug, Clone)]
pub struct GetPullRequestAttachmentListParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
    pub number: PullRequestNumber,
}

impl GetPullRequestAttachmentListParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        number: impl Into<PullRequestNumber>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            number: number.into(),
        }
    }
}

impl IntoRequest for GetPullRequestAttachmentListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments",
            self.project_id_or_key, self.repo_id_or_name, self.number
        )
    }
}
