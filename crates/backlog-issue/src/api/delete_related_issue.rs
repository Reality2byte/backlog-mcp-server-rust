use crate::models::RelatedIssue;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{IssueIdOrKey, identifier::IssueId};

pub type DeleteRelatedIssueResponse = RelatedIssue;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteRelatedIssueParams {
    pub issue_id_or_key: IssueIdOrKey,
    pub related_issue_id: IssueId,
}

impl DeleteRelatedIssueParams {
    pub fn new(
        issue_id_or_key: impl Into<IssueIdOrKey>,
        related_issue_id: impl Into<IssueId>,
    ) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            related_issue_id: related_issue_id.into(),
        }
    }
}

impl IntoRequest for DeleteRelatedIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/relatedIssues/{}",
            self.issue_id_or_key, self.related_issue_id
        )
    }
}
