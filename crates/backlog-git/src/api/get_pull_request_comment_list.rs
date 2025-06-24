use crate::models::{PrCommentOrder, PullRequestComment};
use backlog_api_core::IntoRequest;
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName, identifier::PullRequestNumber};
use serde::Serialize;

pub type GetPullRequestCommentListResponse = Vec<PullRequestComment>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPullRequestCommentListParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub repo_id_or_name: RepositoryIdOrName,
    #[serde(skip)]
    pub number: PullRequestNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<PrCommentOrder>,
}

impl GetPullRequestCommentListParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        number: impl Into<PullRequestNumber>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            number: number.into(),
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        }
    }

    pub fn min_id(mut self, min_id: u32) -> Self {
        self.min_id = Some(min_id);
        self
    }

    pub fn max_id(mut self, max_id: u32) -> Self {
        self.max_id = Some(max_id);
        self
    }

    pub fn count(mut self, count: u8) -> Self {
        self.count = Some(count);
        self
    }

    pub fn order(mut self, order: PrCommentOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl IntoRequest for GetPullRequestCommentListParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
            self.project_id_or_key, self.repo_id_or_name, self.number
        )
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
