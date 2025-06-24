use backlog_api_core::IntoDownloadRequest;

/// Parameters for downloading document attachment
///
/// Corresponds to `GET /api/v2/documents/:documentId/attachments/:attachmentId`.
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadAttachmentParams {
    pub document_id: backlog_core::identifier::DocumentId,
    pub attachment_id: backlog_core::identifier::DocumentAttachmentId,
}

impl DownloadAttachmentParams {
    pub fn new(
        document_id: impl Into<backlog_core::identifier::DocumentId>,
        attachment_id: impl Into<backlog_core::identifier::DocumentAttachmentId>,
    ) -> Self {
        Self {
            document_id: document_id.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl IntoDownloadRequest for DownloadAttachmentParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/documents/{}/attachments/{}",
            self.document_id, self.attachment_id
        )
    }
}
