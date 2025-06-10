use std::str::FromStr;
use std::sync::Arc;

use tokio::sync::Mutex;

use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    DocumentDetail, DocumentTreeResponse, DownloadedFile, GetDocumentTreeParams,
};
use backlog_core::{
    ProjectIdOrKey,
    identifier::{DocumentAttachmentId, DocumentId},
};

use super::request::{
    DownloadDocumentAttachmentRequest, GetDocumentDetailsRequest, GetDocumentTreeRequest,
};
use crate::error::Result;

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
) -> Result<DownloadedFile> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;
    let attachment_id = DocumentAttachmentId::new(req.attachment_id);

    client_guard
        .document()
        .download_attachment(document_id, attachment_id)
        .await
        .map_err(crate::error::Error::from)
}

pub(crate) async fn get_document_tree_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentTreeRequest,
) -> Result<DocumentTreeResponse> {
    let client_guard = client.lock().await;
    let project_id_or_key_val = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;
    // Construct directly instead of using the builder, to sidestep the E0599 error for now.
    let params = GetDocumentTreeParams {
        project_id_or_key: project_id_or_key_val,
    };

    client_guard
        .document()
        .get_document_tree(params)
        .await
        .map_err(crate::error::Error::from)
}
