#[cfg(feature = "writable")]
use crate::models::Comment;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{
    IssueIdOrKey,
    identifier::{AttachmentId, UserId},
};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Response type for adding a comment
#[cfg(feature = "writable")]
pub type AddCommentResponse = Comment;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct AddCommentParams {
    #[builder(setter(into))]
    #[form(skip)]
    pub issue_id_or_key: IssueIdOrKey,
    #[builder(setter(into))]
    pub content: String,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "notifiedUserId")]
    pub notified_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "attachmentId")]
    pub attachment_id: Option<Vec<AttachmentId>>,
}

#[cfg(feature = "writable")]
impl IntoRequest for AddCommentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}/comments", self.issue_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
