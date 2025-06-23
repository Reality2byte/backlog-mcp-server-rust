use super::{
    DownloadAttachmentParams, GetDocumentParams, GetDocumentTreeParams, GetDocumentTreeResponse,
    ListDocumentsParams, ListDocumentsResponse,
};
use crate::models::DocumentDetail;
use backlog_api_core::Result;
use client::{Client, DownloadedFile};

pub struct DocumentApi(Client);

impl DocumentApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get documents
    ///
    /// Corresponds to `GET /api/v2/documents`.
    pub async fn list_documents(
        &self,
        params: ListDocumentsParams,
    ) -> Result<ListDocumentsResponse> {
        self.0.execute(params).await
    }

    /// Get document tree
    ///
    /// Corresponds to `GET /api/v2/documents/tree`.
    pub async fn get_document_tree(
        &self,
        params: GetDocumentTreeParams,
    ) -> Result<GetDocumentTreeResponse> {
        self.0.execute(params).await
    }

    /// Get document
    ///
    /// Corresponds to `GET /api/v2/documents/:documentId`.
    pub async fn get_document(&self, params: GetDocumentParams) -> Result<DocumentDetail> {
        self.0.execute(params).await
    }

    /// Get document attachment
    ///
    /// Corresponds to `GET /api/v2/documents/:documentId/attachments/:attachmentId`.
    pub async fn download_attachment(
        &self,
        params: DownloadAttachmentParams,
    ) -> Result<DownloadedFile> {
        self.0.download_file(params).await
    }
}
