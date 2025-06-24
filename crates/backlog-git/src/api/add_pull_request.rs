use crate::models::PullRequest;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{AttachmentId, IssueId, UserId},
};
use serde::Serialize;

#[cfg(feature = "macros")]
use backlog_api_macros::ToFormParams;

pub type AddPullRequestResponse = PullRequest;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "macros", derive(ToFormParams))]
pub struct AddPullRequestParams {
    #[cfg_attr(feature = "macros", form(skip))]
    pub project_id_or_key: ProjectIdOrKey,
    #[cfg_attr(feature = "macros", form(skip))]
    pub repo_id_or_name: RepositoryIdOrName,
    pub summary: String,
    pub description: String,
    pub base: String,
    pub branch: String,
    #[cfg_attr(feature = "macros", form(name = "issueId"))]
    pub issue_id: Option<IssueId>,
    #[cfg_attr(feature = "macros", form(name = "assigneeId"))]
    pub assignee_id: Option<UserId>,
    #[cfg_attr(feature = "macros", form(array, name = "notifiedUserId"))]
    pub notified_user_ids: Option<Vec<UserId>>,
    #[cfg_attr(feature = "macros", form(array, name = "attachmentId"))]
    pub attachment_ids: Option<Vec<AttachmentId>>,
}

impl AddPullRequestParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        summary: impl Into<String>,
        description: impl Into<String>,
        base: impl Into<String>,
        branch: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            repo_id_or_name: repo_id_or_name.into(),
            summary: summary.into(),
            description: description.into(),
            base: base.into(),
            branch: branch.into(),
            issue_id: None,
            assignee_id: None,
            notified_user_ids: None,
            attachment_ids: None,
        }
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

    pub fn attachment_ids(mut self, attachment_ids: Vec<AttachmentId>) -> Self {
        self.attachment_ids = Some(attachment_ids);
        self
    }
}

// Form serialization: macro when available, manual fallback
#[cfg(not(feature = "macros"))]
impl From<&AddPullRequestParams> for Vec<(String, String)> {
    fn from(params: &AddPullRequestParams) -> Self {
        let mut seq = vec![
            ("summary".to_string(), params.summary.clone()),
            ("description".to_string(), params.description.clone()),
            ("base".to_string(), params.base.clone()),
            ("branch".to_string(), params.branch.clone()),
        ];

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

        if let Some(attachment_ids) = &params.attachment_ids {
            attachment_ids.iter().for_each(|id| {
                seq.push(("attachmentId[]".to_string(), id.value().to_string()));
            });
        }

        seq
    }
}

impl IntoRequest for AddPullRequestParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests",
            self.project_id_or_key, self.repo_id_or_name
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
