#[cfg(feature = "writable")]
use crate::models::Comment;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::IssueIdOrKey;
use backlog_core::identifier::CommentId;
use serde::Serialize;

#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;

#[cfg(feature = "writable")]
pub type UpdateCommentResponse = Comment;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct UpdateCommentParams {
    #[form(skip)]
    pub issue_id_or_key: IssueIdOrKey,
    #[form(skip)]
    pub comment_id: CommentId,
    pub content: String,
}

#[cfg(feature = "writable")]
impl UpdateCommentParams {
    pub fn new(
        issue_id_or_key: impl Into<IssueIdOrKey>,
        comment_id: impl Into<CommentId>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            comment_id: comment_id.into(),
            content: content.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateCommentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/comments/{}",
            self.issue_id_or_key, self.comment_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
