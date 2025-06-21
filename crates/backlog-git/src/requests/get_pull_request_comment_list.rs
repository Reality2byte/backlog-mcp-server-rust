use backlog_api_core::{Error as ApiError, IntoRequest, Result};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, PullRequestNumber},
};
use derive_builder::Builder;
use reqwest::Client as ReqwestClient;
use serde::Serialize;
use url::Url;

use crate::models::PrCommentOrder;

#[derive(Builder, Debug, Serialize, Clone)]
#[builder(build_fn(error = "ApiError"))]
#[serde(rename_all = "camelCase")]
pub struct GetPullRequestCommentListParams {
    /// The project ID or key where the repository is located.
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    #[serde(skip)]
    pub repo_id_or_name: RepositoryIdOrName,
    /// The pull request number.
    #[serde(skip)]
    pub pr_number: PullRequestNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<PrCommentOrder>,
}

impl GetPullRequestCommentListParams {
    /// Creates a new instance with the required path fields and all optional query parameters set to None.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            pr_number,
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        }
    }
}

impl IntoRequest for GetPullRequestCommentListParams {
    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
            self.project_id_or_key,
            self.repo_id_or_name,
            self.pr_number.value()
        );
        self.get(client, base_url, path, &self)
    }
}
