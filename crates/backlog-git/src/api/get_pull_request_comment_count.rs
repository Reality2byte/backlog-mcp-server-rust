use crate::models::PullRequestCommentCount;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName, identifier::PullRequestNumber};

pub type GetPullRequestCommentCountResponse = PullRequestCommentCount;

#[derive(Debug, Clone)]
pub struct GetPullRequestCommentCountParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
    pub number: PullRequestNumber,
}

impl GetPullRequestCommentCountParams {
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

impl IntoRequest for GetPullRequestCommentCountParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/count",
            self.project_id_or_key, self.repo_id_or_name, self.number
        )
    }
}
