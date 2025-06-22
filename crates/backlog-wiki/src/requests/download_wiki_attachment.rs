use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::{Identifier, WikiAttachmentId, WikiId};
use derive_builder::Builder;

/// Parameters for downloading a wiki attachment.
///
/// Corresponds to `GET /api/v2/wikis/:wikiId/attachments/:attachmentId`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "backlog_api_core::Error"))]
pub struct DownloadWikiAttachmentParams {
    /// The wiki ID.
    pub wiki_id: WikiId,
    /// The attachment ID.
    pub attachment_id: WikiAttachmentId,
}

impl DownloadWikiAttachmentParams {
    /// Creates a new instance with the required parameters.
    pub fn new(wiki_id: impl Into<WikiId>, attachment_id: impl Into<WikiAttachmentId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl IntoRequest for DownloadWikiAttachmentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/wikis/{}/attachments/{}",
            self.wiki_id.value(),
            self.attachment_id.value()
        )
    }
}
