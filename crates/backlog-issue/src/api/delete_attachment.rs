#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::IssueIdOrKey;
#[cfg(feature = "writable")]
use backlog_core::identifier::AttachmentId;

#[cfg(feature = "writable")]
use crate::models::Attachment;

/// Response type for delete attachment operations.
#[cfg(feature = "writable")]
pub type DeleteAttachmentResponse = Attachment;

/// Parameters for deleting an attachment from an issue.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteAttachmentParams {
    pub issue_id_or_key: IssueIdOrKey,
    pub attachment_id: AttachmentId,
}

#[cfg(feature = "writable")]
impl DeleteAttachmentParams {
    /// Creates new parameters for deleting an attachment.
    pub fn new(
        issue_id_or_key: impl Into<IssueIdOrKey>,
        attachment_id: impl Into<AttachmentId>,
    ) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteAttachmentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/attachments/{}",
            self.issue_id_or_key, self.attachment_id
        )
    }
}
