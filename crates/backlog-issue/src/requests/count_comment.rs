use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::IssueIdOrKey;
use serde::Serialize;

/// Parameters for counting comments for an issue.
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/count`.
#[derive(Debug, Clone)]
pub struct CountCommentParams {
    pub issue_id_or_key: IssueIdOrKey,
}

impl CountCommentParams {
    /// Creates a new instance.
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

impl IntoRequest for CountCommentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}/comments/count", self.issue_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        // No query parameters needed for this endpoint
        Vec::<(String, String)>::new()
    }
}
