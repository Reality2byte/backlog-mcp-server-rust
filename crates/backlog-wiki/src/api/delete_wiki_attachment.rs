use crate::models::WikiAttachment;
/// Corresponds to `DELETE /api/v2/wikis/:wikiId/attachments/:attachmentId`.
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::{WikiAttachmentId, WikiId};

/// Type alias for the response of `delete_wiki_attachment` API.
pub type DeleteWikiAttachmentResponse = WikiAttachment;

/// Parameters for deleting a wiki attachment.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteWikiAttachmentParams {
    pub wiki_id: WikiId,
    pub attachment_id: WikiAttachmentId,
}

#[cfg(feature = "writable")]
impl DeleteWikiAttachmentParams {
    /// Creates a new instance of `DeleteWikiAttachmentParams` with required parameters.
    pub fn new(wiki_id: WikiId, attachment_id: WikiAttachmentId) -> Self {
        Self {
            wiki_id,
            attachment_id,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteWikiAttachmentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/wikis/{}/attachments/{}",
            self.wiki_id, self.attachment_id
        )
    }
}
