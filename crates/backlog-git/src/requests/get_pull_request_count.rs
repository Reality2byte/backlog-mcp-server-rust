use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName};
use derive_builder::Builder;
use serde::Serialize;

/// Parameters for getting pull request count.
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/count`.
#[derive(Builder, Debug, Clone, Serialize)]
#[builder(build_fn(error = "ApiError"))]
#[serde(rename_all = "camelCase")]
pub struct GetPullRequestCountParams {
    /// The project ID or key.
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    #[serde(skip)]
    pub repo_id_or_name: RepositoryIdOrName,
    /// Filter by status IDs.
    #[builder(default)]
    #[serde(rename = "statusId[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_ids: Option<Vec<u32>>,
    /// Filter by assignee IDs.
    #[builder(default)]
    #[serde(rename = "assigneeId[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<u32>>,
    /// Filter by issue IDs.
    #[builder(default)]
    #[serde(rename = "issueId[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_ids: Option<Vec<u32>>,
    /// Filter by created user IDs.
    #[builder(default)]
    #[serde(rename = "createdUserId[]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_user_ids: Option<Vec<u32>>,
}

impl GetPullRequestCountParams {
    /// Creates a new instance with the required parameters.
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            status_ids: None,
            assignee_ids: None,
            issue_ids: None,
            created_user_ids: None,
        }
    }

    /// Convert parameters to query parameter vector for HTTP requests.
    ///
    /// Handles array parameters with [] notation as required by Backlog API.
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        // Handle status ID array parameters
        if let Some(status_ids) = &self.status_ids {
            status_ids.iter().for_each(|id| {
                params.push(("statusId[]".to_string(), id.to_string()));
            });
        }

        // Handle assignee ID array parameters
        if let Some(assignee_ids) = &self.assignee_ids {
            assignee_ids.iter().for_each(|id| {
                params.push(("assigneeId[]".to_string(), id.to_string()));
            });
        }

        // Handle issue ID array parameters
        if let Some(issue_ids) = &self.issue_ids {
            issue_ids.iter().for_each(|id| {
                params.push(("issueId[]".to_string(), id.to_string()));
            });
        }

        // Handle created user ID array parameters
        if let Some(created_user_ids) = &self.created_user_ids {
            created_user_ids.iter().for_each(|id| {
                params.push(("createdUserId[]".to_string(), id.to_string()));
            });
        }

        params
    }
}

impl IntoRequest for GetPullRequestCountParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/count",
            self.project_id_or_key, self.repo_id_or_name
        )
    }

    fn to_query(&self) -> impl Serialize {
        self.to_query_params()
    }
}
