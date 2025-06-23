use backlog_api_core::IntoDownloadRequest;
use backlog_core::identifier::{Identifier, WikiAttachmentId, WikiId};

#[derive(Debug, Clone)]
pub struct DownloadWikiAttachmentParams {
    pub wiki_id: WikiId,
    pub attachment_id: WikiAttachmentId,
}

impl DownloadWikiAttachmentParams {
    pub fn new(wiki_id: impl Into<WikiId>, attachment_id: impl Into<WikiAttachmentId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl IntoDownloadRequest for DownloadWikiAttachmentParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/wikis/{}/attachments/{}",
            self.wiki_id.value(),
            self.attachment_id.value()
        )
    }
}
