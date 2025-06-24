use crate::models::PullRequestComment;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{PullRequestCommentId, PullRequestNumber},
};
use serde::Serialize;

pub type UpdatePullRequestCommentResponse = PullRequestComment;

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePullRequestCommentParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub repo_id_or_name: RepositoryIdOrName,
    #[serde(skip)]
    pub number: PullRequestNumber,
    #[serde(skip)]
    pub comment_id: PullRequestCommentId,
    pub content: String,
}

impl UpdatePullRequestCommentParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        number: impl Into<PullRequestNumber>,
        comment_id: impl Into<PullRequestCommentId>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            number: number.into(),
            comment_id: comment_id.into(),
            content: content.into(),
        }
    }
}

impl IntoRequest for UpdatePullRequestCommentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/{}",
            self.project_id_or_key, self.repo_id_or_name, self.number, self.comment_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
