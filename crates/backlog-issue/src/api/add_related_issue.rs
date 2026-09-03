use crate::models::RelatedIssue;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{IssueIdOrKey, identifier::IssueId};
use serde::Serialize;

pub type AddRelatedIssueResponse = RelatedIssue;

#[derive(Debug, Clone, PartialEq)]
pub struct AddRelatedIssueParams {
    pub issue_id_or_key: IssueIdOrKey,
    pub target_issue_id: IssueId,
}

impl AddRelatedIssueParams {
    pub fn new(
        issue_id_or_key: impl Into<IssueIdOrKey>,
        target_issue_id: impl Into<IssueId>,
    ) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            target_issue_id: target_issue_id.into(),
        }
    }
}

impl From<&AddRelatedIssueParams> for Vec<(String, String)> {
    fn from(params: &AddRelatedIssueParams) -> Self {
        vec![(
            "targetIssueId".to_string(),
            params.target_issue_id.to_string(),
        )]
    }
}

impl IntoRequest for AddRelatedIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}/relatedIssues", self.issue_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
