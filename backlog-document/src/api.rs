use backlog_api_core::{Error as ApiError, Result};
use backlog_core::DocumentId;
use backlog_core::Identifier;
use backlog_core::identifier::AttachmentId;
use client::Client;
use reqwest; // For attachment download

use crate::models::{Document, DocumentDetail, DocumentTreeResponse};
use crate::requests::{GetDocumentTreeParams, ListDocumentsParams};

pub struct DocumentApi(Client);

impl DocumentApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get documents
    /// GET /api/v2/documents
    pub async fn list_documents(&self, params: ListDocumentsParams) -> Result<Vec<Document>> {
        let query_params: Vec<(String, String)> = params.into();
        self.0
            .get_with_params("/api/v2/documents", &query_params)
            .await
    }

    /// Get document tree
    /// GET /api/v2/documents/tree
    pub async fn get_document_tree(
        &self,
        params: GetDocumentTreeParams,
    ) -> Result<DocumentTreeResponse> {
        let query_params: Vec<(String, String)> = params.into();
        self.0
            .get_with_params("/api/v2/documents/tree", &query_params)
            .await
    }

    /// Get document
    /// GET /api/v2/documents/:documentId
    pub async fn get_document(&self, document_id: impl Into<DocumentId>) -> Result<DocumentDetail> {
        let path = format!("/api/v2/documents/{}", document_id.into());
        self.0.get(&path).await
    }

    /// Get document attachment
    /// GET /api/v2/documents/:documentId/attachments/:attachmentId
    pub async fn download_attachment(
        &self,
        document_id: impl Into<DocumentId>,
        attachment_id: impl Into<AttachmentId>,
    ) -> Result<reqwest::Response> {
        let _path = format!(
            "/api/v2/documents/{}/attachments/{}",
            document_id.into().value(),
            attachment_id.into().value()
        );
        // Placeholder: Actual implementation requires modification to client::Client
        // to expose raw reqwest::Response or a streaming body.
        Err(ApiError::Client(
            "download_attachment feature not fully implemented: requires client modification"
                .to_string(),
        ))
    }
}
