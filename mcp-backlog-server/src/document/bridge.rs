use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{DocumentDetail, DocumentId};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::request::{DownloadDocumentAttachmentRequest, GetDocumentDetailsRequest}; // Added DownloadDocumentAttachmentRequest
use crate::error::Result;
use backlog_api_client::bytes;
use backlog_core::identifier::DocumentAttachmentId; // Changed to backlog_core::identifier // Changed to backlog_api_client re-export

pub(crate) async fn get_document_details(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentDetailsRequest,
) -> Result<DocumentDetail> {
    let client = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;
    let document = client.document().get_document(document_id.clone()).await?;
    Ok(document)
}

pub(crate) async fn download_document_attachment_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DownloadDocumentAttachmentRequest,
) -> Result<(String, String, bytes::Bytes)> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;
    let attachment_id = DocumentAttachmentId::new(req.attachment_id);

    client_guard
        .document()
        .download_attachment(document_id, attachment_id)
        .await
        .map_err(crate::error::Error::from)
}
