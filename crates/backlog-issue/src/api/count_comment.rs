use backlog_api_core::IntoRequest;
use backlog_core::IssueIdOrKey;
use serde::{Deserialize, Serialize};

/// Response type for counting comments
#[derive(Debug, Deserialize)]
pub struct CountCommentResponse {
    pub count: u32,
}

/// Parameters for counting comments for a specific issue.
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/count`.
#[derive(Debug, Clone, PartialEq)]
pub struct CountCommentParams {
    pub issue_id_or_key: IssueIdOrKey,
}

impl CountCommentParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

impl IntoRequest for CountCommentParams {
    fn path(&self) -> String {
        format!("/api/v2/issues/{}/comments/count", self.issue_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
