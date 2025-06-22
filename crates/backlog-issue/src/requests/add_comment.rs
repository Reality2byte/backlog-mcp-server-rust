use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
use backlog_core::{
    IssueIdOrKey,
    identifier::{AttachmentId, Identifier, UserId},
};
use derive_builder::Builder;
use serde::Serialize;

/// Parameters for [IssueApi::add_comment](crate::api::IssueApi::add_comment).
///
/// Used to add a new comment to an existing issue.
/// Use the associated builder `AddCommentParamsBuilder` to construct an instance.
///
/// # Example
///
/// ```
/// use backlog_issue::requests::add_comment::AddCommentParamsBuilder;
/// use backlog_core::identifier::{UserId, AttachmentId};
///
/// let params = AddCommentParamsBuilder::default()
///     .issue_id_or_key("TEST-1")
///     .content("This is a new comment")
///     .notified_user_id(vec![UserId::new(123), UserId::new(456)])
///     .attachment_id(vec![AttachmentId::new(789)])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into), build_fn(error = "ApiError"))]
pub struct AddCommentParams {
    /// The issue ID or key to add the comment to.
    #[builder(setter(into))]
    pub issue_id_or_key: IssueIdOrKey,

    /// The content of the comment (required).
    #[builder(setter(into))]
    pub content: String,

    /// User IDs to notify about this comment (optional).
    #[builder(default, setter(into, strip_option))]
    pub notified_user_id: Option<Vec<UserId>>,

    /// Attachment IDs to include with this comment (optional).
    #[builder(default, setter(into, strip_option))]
    pub attachment_id: Option<Vec<AttachmentId>>,
}

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

impl From<&AddCommentParams> for Vec<(String, String)> {
    fn from(params: &AddCommentParams) -> Self {
        let mut seq = Vec::new();

        // Add content (required)
        seq.push(("content".to_string(), params.content.clone()));

        // Add notified user IDs (if any)
        if let Some(ref user_ids) = params.notified_user_id {
            for user_id in user_ids {
                seq.push(("notifiedUserId[]".to_string(), user_id.value().to_string()));
            }
        }

        // Add attachment IDs (if any)
        if let Some(ref attachment_ids) = params.attachment_id {
            for attachment_id in attachment_ids {
                seq.push((
                    "attachmentId[]".to_string(),
                    attachment_id.value().to_string(),
                ));
            }
        }

        seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_comment_params_content_only() {
        let params = AddCommentParamsBuilder::default()
            .issue_id_or_key("TEST-1".parse::<backlog_core::IssueKey>().unwrap())
            .content("Test comment")
            .build()
            .unwrap();

        assert_eq!(params.content, "Test comment");
        assert!(params.notified_user_id.is_none());
        assert!(params.attachment_id.is_none());
    }

    #[test]
    fn test_add_comment_params_with_notifications_and_attachments() {
        let params = AddCommentParamsBuilder::default()
            .issue_id_or_key("TEST-2".parse::<backlog_core::IssueKey>().unwrap())
            .content("Test comment with notifications")
            .notified_user_id(vec![UserId::new(123), UserId::new(456)])
            .attachment_id(vec![AttachmentId::new(789), AttachmentId::new(101112)])
            .build()
            .unwrap();

        assert_eq!(params.content, "Test comment with notifications");
        assert_eq!(params.notified_user_id.as_ref().unwrap().len(), 2);
        assert_eq!(params.attachment_id.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_add_comment_params_builder_error_on_missing_content() {
        let result = AddCommentParamsBuilder::default()
            .issue_id_or_key("TEST-3".parse::<backlog_core::IssueKey>().unwrap())
            .notified_user_id(vec![UserId::new(123)])
            .build();

        assert!(result.is_err());
    }
}
