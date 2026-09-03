use crate::models::RelatedIssue;
use backlog_api_core::IntoRequest;
use backlog_core::IssueIdOrKey;

pub type GetRelatedIssuesResponse = Vec<RelatedIssue>;

#[derive(Debug, Clone, PartialEq)]
pub struct GetRelatedIssuesParams {
    pub issue_id_or_key: IssueIdOrKey,
}

impl GetRelatedIssuesParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetRelatedIssuesParams {
    fn path(&self) -> String {
        format!("/api/v2/issues/{}/relatedIssues", self.issue_id_or_key)
    }
}
