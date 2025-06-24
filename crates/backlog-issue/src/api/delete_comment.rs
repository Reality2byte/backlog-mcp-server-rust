#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::IssueIdOrKey;
#[cfg(feature = "writable")]
use backlog_core::identifier::CommentId;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
use crate::models::Comment;

/// Response type for delete comment operations.
#[cfg(feature = "writable")]
pub type DeleteCommentResponse = Comment;

/// Parameters for deleting a comment from an issue.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteCommentParams {
    pub issue_id_or_key: IssueIdOrKey,
    pub comment_id: CommentId,
}

#[cfg(feature = "writable")]
impl DeleteCommentParams {
    /// Creates new parameters for deleting a comment.
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>, comment_id: impl Into<CommentId>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            comment_id: comment_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteCommentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/comments/{}",
            self.issue_id_or_key, self.comment_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        // DELETE operations don't require a request body
        let params: Vec<(String, String)> = Vec::new();
        params
    }
}
