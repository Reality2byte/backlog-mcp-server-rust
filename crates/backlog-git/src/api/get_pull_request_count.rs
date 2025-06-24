use crate::models::PullRequestCount;
use backlog_api_core::IntoRequest;
#[cfg(feature = "macros")]
use backlog_api_macros::ToFormParams;
#[cfg(not(feature = "macros"))]
use backlog_core::identifier::Identifier;
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{IssueId, StatusId, UserId},
};
use serde::Serialize;

pub type GetPullRequestCountResponse = PullRequestCount;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "macros", derive(ToFormParams))]
pub struct GetPullRequestCountParams {
    #[cfg_attr(feature = "macros", form(skip))]
    pub project_id_or_key: ProjectIdOrKey,
    #[cfg_attr(feature = "macros", form(skip))]
    pub repo_id_or_name: RepositoryIdOrName,
    #[cfg_attr(feature = "macros", form(array, name = "statusId"))]
    pub status_ids: Option<Vec<StatusId>>,
    #[cfg_attr(feature = "macros", form(array, name = "assigneeId"))]
    pub assignee_ids: Option<Vec<UserId>>,
    #[cfg_attr(feature = "macros", form(array, name = "issueId"))]
    pub issue_ids: Option<Vec<IssueId>>,
    #[cfg_attr(feature = "macros", form(array, name = "createdUserId"))]
    pub created_user_ids: Option<Vec<UserId>>,
}

impl GetPullRequestCountParams {
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

    pub fn status_ids(mut self, status_ids: Vec<StatusId>) -> Self {
        self.status_ids = Some(status_ids);
        self
    }

    pub fn assignee_ids(mut self, assignee_ids: Vec<UserId>) -> Self {
        self.assignee_ids = Some(assignee_ids);
        self
    }

    pub fn issue_ids(mut self, issue_ids: Vec<IssueId>) -> Self {
        self.issue_ids = Some(issue_ids);
        self
    }

    pub fn created_user_ids(mut self, created_user_ids: Vec<UserId>) -> Self {
        self.created_user_ids = Some(created_user_ids);
        self
    }
}

impl IntoRequest for GetPullRequestCountParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/count",
            self.project_id_or_key, self.repo_id_or_name
        )
    }

    fn to_query(&self) -> impl Serialize {
        #[cfg(feature = "macros")]
        {
            let params: Vec<(String, String)> = self.into();
            params
        }

        #[cfg(not(feature = "macros"))]
        {
            let mut params = Vec::new();

            if let Some(status_ids) = &self.status_ids {
                status_ids.iter().for_each(|id| {
                    params.push(("statusId[]".to_string(), id.value().to_string()));
                });
            }

            if let Some(assignee_ids) = &self.assignee_ids {
                assignee_ids.iter().for_each(|id| {
                    params.push(("assigneeId[]".to_string(), id.value().to_string()));
                });
            }

            if let Some(issue_ids) = &self.issue_ids {
                issue_ids.iter().for_each(|id| {
                    params.push(("issueId[]".to_string(), id.value().to_string()));
                });
            }

            if let Some(created_user_ids) = &self.created_user_ids {
                created_user_ids.iter().for_each(|id| {
                    params.push(("createdUserId[]".to_string(), id.value().to_string()));
                });
            }

            params
        }
    }
}
