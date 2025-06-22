use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{
    IssueIdOrKey,
    identifier::{CommentId, Identifier},
};
use serde::Serialize;

/// Parameters for getting a specific comment for an issue.
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/:commentId`.
#[derive(Debug, Clone)]
pub struct GetCommentParams {
    pub issue_id_or_key: IssueIdOrKey,
    pub comment_id: CommentId,
}

impl GetCommentParams {
    /// Creates a new instance.
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>, comment_id: CommentId) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            comment_id,
        }
    }
}

impl IntoRequest for GetCommentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/comments/{}",
            self.issue_id_or_key,
            self.comment_id.value()
        )
    }

    fn to_query(&self) -> impl Serialize {
        // No query parameters needed for this endpoint
        Vec::<(String, String)>::new()
    }
}
