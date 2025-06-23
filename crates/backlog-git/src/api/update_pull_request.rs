use crate::models::PullRequest;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, IssueId, PullRequestNumber, UserId},
};
use serde::Serialize;

pub type UpdatePullRequestResponse = PullRequest;

#[derive(Debug, Clone)]
pub struct UpdatePullRequestParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub repo_id_or_name: RepositoryIdOrName,
    pub number: PullRequestNumber,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub issue_id: Option<IssueId>,
    pub assignee_id: Option<UserId>,
    pub notified_user_ids: Option<Vec<UserId>>,
    pub comment: Option<String>,
}

impl UpdatePullRequestParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        number: impl Into<PullRequestNumber>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            number: number.into(),
            summary: None,
            description: None,
            issue_id: None,
            assignee_id: None,
            notified_user_ids: None,
            comment: None,
        }
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn issue_id(mut self, issue_id: IssueId) -> Self {
        self.issue_id = Some(issue_id);
        self
    }

    pub fn assignee_id(mut self, assignee_id: UserId) -> Self {
        self.assignee_id = Some(assignee_id);
        self
    }

    pub fn notified_user_ids(mut self, notified_user_ids: Vec<UserId>) -> Self {
        self.notified_user_ids = Some(notified_user_ids);
        self
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

impl From<&UpdatePullRequestParams> for Vec<(String, String)> {
    fn from(params: &UpdatePullRequestParams) -> Self {
        let mut seq = Vec::new();

        if let Some(summary) = &params.summary {
            seq.push(("summary".to_string(), summary.clone()));
        }

        if let Some(description) = &params.description {
            seq.push(("description".to_string(), description.clone()));
        }

        if let Some(issue_id) = &params.issue_id {
            seq.push(("issueId".to_string(), issue_id.value().to_string()));
        }

        if let Some(assignee_id) = &params.assignee_id {
            seq.push(("assigneeId".to_string(), assignee_id.value().to_string()));
        }

        if let Some(user_ids) = &params.notified_user_ids {
            user_ids.iter().for_each(|id| {
                seq.push(("notifiedUserId[]".to_string(), id.value().to_string()));
            });
        }

        if let Some(comment) = &params.comment {
            seq.push(("comment".to_string(), comment.clone()));
        }

        seq
    }
}

impl IntoRequest for UpdatePullRequestParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
            self.project_id_or_key, self.repo_id_or_name, self.number
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
