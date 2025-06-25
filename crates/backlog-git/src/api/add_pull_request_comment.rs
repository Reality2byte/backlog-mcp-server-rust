use crate::models::PullRequestComment;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{PullRequestNumber, UserId},
};
use serde::Serialize;

use backlog_api_macros::ToFormParams;

pub type AddPullRequestCommentResponse = PullRequestComment;

#[derive(Debug, Clone, ToFormParams)]
pub struct AddPullRequestCommentParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[form(skip)]
    pub repo_id_or_name: RepositoryIdOrName,
    #[form(skip)]
    pub number: PullRequestNumber,
    pub content: String,
    #[form(array, name = "notifiedUserId")]
    pub notified_user_ids: Option<Vec<UserId>>,
}

impl AddPullRequestCommentParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        number: impl Into<PullRequestNumber>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            number: number.into(),
            content: content.into(),
            notified_user_ids: None,
        }
    }

    pub fn notified_user_ids(mut self, notified_user_ids: Vec<UserId>) -> Self {
        self.notified_user_ids = Some(notified_user_ids);
        self
    }
}

impl IntoRequest for AddPullRequestCommentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
            self.project_id_or_key, self.repo_id_or_name, self.number
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
