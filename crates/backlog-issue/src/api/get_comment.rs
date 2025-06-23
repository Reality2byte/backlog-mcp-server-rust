use crate::models::Comment;
use backlog_api_core::IntoRequest;
use backlog_core::{IssueIdOrKey, identifier::CommentId};

/// Response type for getting a specific comment
pub type GetCommentResponse = Comment;

/// Parameters for getting a specific comment.
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/comments/:commentId`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetCommentParams {
    pub issue_id_or_key: IssueIdOrKey,
    pub comment_id: CommentId,
}

impl GetCommentParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>, comment_id: impl Into<CommentId>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            comment_id: comment_id.into(),
        }
    }
}

impl IntoRequest for GetCommentParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/comments/{}",
            self.issue_id_or_key, self.comment_id
        )
    }
}
