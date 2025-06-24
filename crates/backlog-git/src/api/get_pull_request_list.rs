use crate::models::PullRequest;
use backlog_api_core::IntoRequest;
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, IssueId, StatusId, UserId},
};
use serde::Serialize;

pub type GetPullRequestListResponse = Vec<PullRequest>;

#[derive(Debug, Clone)]
pub struct GetPullRequestListParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
    pub status_ids: Option<Vec<StatusId>>,
    pub assignee_ids: Option<Vec<UserId>>,
    pub issue_ids: Option<Vec<IssueId>>,
    pub created_user_ids: Option<Vec<UserId>>,
    pub offset: Option<u32>,
    pub count: Option<u8>,
}

impl GetPullRequestListParams {
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

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn count(mut self, count: u8) -> Self {
        self.count = Some(count);
        self
    }
}

impl IntoRequest for GetPullRequestListParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests",
            self.project_id_or_key, self.repo_id_or_name
        )
    }

    fn to_query(&self) -> impl Serialize {
        let mut params = Vec::new();

        if let Some(status_ids) = &self.status_ids {
            for id in status_ids {
                params.push(("statusId[]".to_string(), id.value().to_string()));
            }
        }

        if let Some(assignee_ids) = &self.assignee_ids {
            for id in assignee_ids {
                params.push(("assigneeId[]".to_string(), id.value().to_string()));
            }
        }

        if let Some(issue_ids) = &self.issue_ids {
            for id in issue_ids {
                params.push(("issueId[]".to_string(), id.value().to_string()));
            }
        }

        if let Some(created_user_ids) = &self.created_user_ids {
            for id in created_user_ids {
                params.push(("createdUserId[]".to_string(), id.value().to_string()));
            }
        }

        if let Some(offset) = self.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }

        if let Some(count) = self.count {
            params.push(("count".to_string(), count.to_string()));
        }

        params
    }
}
