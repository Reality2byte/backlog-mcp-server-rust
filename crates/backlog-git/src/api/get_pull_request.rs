use crate::models::PullRequest;
use backlog_api_core::IntoRequest;
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName, identifier::PullRequestNumber};

pub type GetPullRequestResponse = PullRequest;

#[derive(Debug, Clone)]
pub struct GetPullRequestParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
    pub number: PullRequestNumber,
}

impl GetPullRequestParams {
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

impl IntoRequest for GetPullRequestParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
            self.project_id_or_key, self.repo_id_or_name, self.number
        )
    }
}
