use crate::DocumentTreeResponse;
use crate::models::{Document, DocumentDetail};
use crate::requests::{GetDocumentTreeParams, ListDocumentsParams};
use backlog_api_core::Result;
use backlog_core::DocumentId;
use backlog_core::Identifier;
use backlog_core::identifier::DocumentAttachmentId;
use client::{Client, DownloadedFile};

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
        attachment_id: impl Into<DocumentAttachmentId>,
    ) -> Result<DownloadedFile> {
        let path = format!(
            "/api/v2/documents/{}/attachments/{}",
            document_id.into().value(),
            attachment_id.into().value()
        );

        self.0.download_file_raw(&path).await // Implemented method body
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_api_core::bytes;
    use backlog_core::identifier::DocumentAttachmentId;
    use client::test_utils::setup_client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_download_attachment_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let doc_api = DocumentApi::new(client);

        let document_id_val = 12345;
        let attachment_id_val = 67890;
        let attachment_content = "This is a document attachment content.";

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/documents/{}/attachments/{}",
                document_id_val, attachment_id_val
            )))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(attachment_content)
                    .insert_header("Content-Type", "application/octet-stream")
                    .insert_header(
                        "Content-Disposition",
                        "attachment; filename=\"doc_attachment.txt\"",
                    ), // Example header
            )
            .mount(&server)
            .await;

        let document_id = DocumentId::new(document_id_val.to_string());
        let attachment_id = DocumentAttachmentId::new(attachment_id_val);

        let result = doc_api
            .download_attachment(document_id, attachment_id)
            .await;

        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, "doc_attachment.txt");
        assert_eq!(downloaded_file.content_type, "application/octet-stream");
        assert_eq!(
            downloaded_file.bytes,
            bytes::Bytes::from(attachment_content)
        );
    }

    #[tokio::test]
    async fn test_download_attachment_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let doc_api = DocumentApi::new(client);

        let document_id_val = 12345;
        let attachment_id_val = 67891;

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/documents/{}/attachments/{}",
                document_id_val, attachment_id_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let document_id = DocumentId::new(document_id_val.to_string());
        let attachment_id = DocumentAttachmentId::new(attachment_id_val);

        let result = doc_api
            .download_attachment(document_id, attachment_id)
            .await;

        assert!(result.is_err());
        // Optionally, check the specific error type if desired
        // match result.unwrap_err() {
        //     backlog_api_core::Error::HttpStatus { status, .. } => {
        //         assert_eq!(status, reqwest::StatusCode::NOT_FOUND)
        //     }
        //     _ => panic!("Expected HttpStatus error"),
        // }
    }
}
