use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, IssueId, StatusId, UserId},
};
use derive_builder::Builder;
use serde::Serialize;

/// Parameters for getting pull request list with filtering options.
///
/// This struct now includes all path information needed to construct the complete request.
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests`.
#[derive(Builder, Debug, Clone, Serialize)]
#[builder(build_fn(error = "ApiError"))]
#[serde(rename_all = "camelCase")]
pub struct GetPullRequestListParams {
    /// The project ID or key where the repository is located.
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    /// The repository ID or name.
    #[serde(skip)]
    pub repo_id_or_name: RepositoryIdOrName,
    /// Filter by pull request status IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub status_ids: Option<Vec<StatusId>>,

    /// Filter by assignee user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub assignee_ids: Option<Vec<UserId>>,

    /// Filter by related issue IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub issue_ids: Option<Vec<IssueId>>,

    /// Filter by creator user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub created_user_ids: Option<Vec<UserId>>,

    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub offset: Option<u32>,

    /// Number of pull requests to retrieve (1-100, default 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub count: Option<u8>,
}

impl GetPullRequestListParams {
    /// Creates a new instance with the required path fields and all optional query parameters set to None.
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
            offset: None,
            count: None,
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
                params.push(("statusId[]".to_string(), id.value().to_string()));
            });
        }

        // Handle assignee ID array parameters
        if let Some(assignee_ids) = &self.assignee_ids {
            assignee_ids.iter().for_each(|id| {
                params.push(("assigneeId[]".to_string(), id.value().to_string()));
            });
        }

        // Handle issue ID array parameters
        if let Some(issue_ids) = &self.issue_ids {
            issue_ids.iter().for_each(|id| {
                params.push(("issueId[]".to_string(), id.value().to_string()));
            });
        }

        // Handle created user ID array parameters
        if let Some(created_user_ids) = &self.created_user_ids {
            created_user_ids.iter().for_each(|id| {
                params.push(("createdUserId[]".to_string(), id.value().to_string()));
            });
        }

        // Handle offset parameter
        if let Some(offset) = self.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }

        // Handle count parameter
        if let Some(count) = self.count {
            params.push(("count".to_string(), count.to_string()));
        }

        params
    }
}

impl IntoRequest for GetPullRequestListParams {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests",
            self.project_id_or_key, self.repo_id_or_name
        )
    }

    fn to_query(&self) -> impl Serialize {
        self.to_query_params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pull_request_list_params_empty() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = GetPullRequestListParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .build()
            .unwrap();
        let query_params = params.to_query_params();
        assert!(query_params.is_empty());
    }

    #[test]
    fn test_get_pull_request_list_params_with_status_ids() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = GetPullRequestListParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .status_ids(vec![StatusId::new(1), StatusId::new(2)])
            .build()
            .unwrap();

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 2);
        assert!(query_params.contains(&("statusId[]".to_string(), "1".to_string())));
        assert!(query_params.contains(&("statusId[]".to_string(), "2".to_string())));
    }

    #[test]
    fn test_get_pull_request_list_params_with_all_params() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = GetPullRequestListParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .status_ids(vec![StatusId::new(1)])
            .assignee_ids(vec![UserId::new(100)])
            .issue_ids(vec![IssueId::new(1000)])
            .created_user_ids(vec![UserId::new(200)])
            .offset(10)
            .count(50)
            .build()
            .unwrap();

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 6);
        assert!(query_params.contains(&("statusId[]".to_string(), "1".to_string())));
        assert!(query_params.contains(&("assigneeId[]".to_string(), "100".to_string())));
        assert!(query_params.contains(&("issueId[]".to_string(), "1000".to_string())));
        assert!(query_params.contains(&("createdUserId[]".to_string(), "200".to_string())));
        assert!(query_params.contains(&("offset".to_string(), "10".to_string())));
        assert!(query_params.contains(&("count".to_string(), "50".to_string())));
    }

    #[test]
    fn test_get_pull_request_list_params_with_multiple_arrays() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params = GetPullRequestListParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .status_ids(vec![StatusId::new(1), StatusId::new(2), StatusId::new(3)])
            .assignee_ids(vec![UserId::new(100), UserId::new(200)])
            .build()
            .unwrap();

        let query_params = params.to_query_params();
        assert_eq!(query_params.len(), 5); // 3 status + 2 assignee

        // Check status IDs
        assert!(query_params.contains(&("statusId[]".to_string(), "1".to_string())));
        assert!(query_params.contains(&("statusId[]".to_string(), "2".to_string())));
        assert!(query_params.contains(&("statusId[]".to_string(), "3".to_string())));

        // Check assignee IDs
        assert!(query_params.contains(&("assigneeId[]".to_string(), "100".to_string())));
        assert!(query_params.contains(&("assigneeId[]".to_string(), "200".to_string())));
    }
}
