use std::str::FromStr;
use std::sync::Arc;

use tokio::sync::Mutex;

use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{
    DocumentDetail, DownloadAttachmentParams, DownloadedFile, GetDocumentParams,
    GetDocumentTreeParams, GetDocumentTreeResponse,
};
use backlog_core::{
    ProjectIdOrKey,
    identifier::{DocumentAttachmentId, DocumentId},
};

use super::request::{
    DownloadDocumentAttachmentRequest, GetDocumentDetailsRequest, GetDocumentTreeRequest,
};
use crate::access_control::AccessControl;
use crate::error::Result;

pub(crate) async fn get_document_details(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentDetailsRequest,
    access_control: &AccessControl,
) -> Result<DocumentDetail> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;
    let params = GetDocumentParams::new(document_id.clone());
    let document = client_guard.document().get_document(params).await?;

    // Check project access
    access_control
        .check_project_access_by_id_async(&document.project_id, &client_guard)
        .await?;

    Ok(document)
}

pub(crate) async fn download_document_attachment_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    req: DownloadDocumentAttachmentRequest,
    access_control: &AccessControl,
) -> Result<DownloadedFile> {
    let client_guard = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim())?;

    // First get document details to check project access
    let document = client_guard
        .document()
        .get_document(GetDocumentParams::new(document_id.clone()))
        .await?;

    // Check project access
    access_control
        .check_project_access_by_id_async(&document.project_id, &client_guard)
        .await?;

    let attachment_id = DocumentAttachmentId::new(req.attachment_id);
    let params = DownloadAttachmentParams::new(document_id, attachment_id);
    client_guard
        .document()
        .download_attachment(params)
        .await
        .map_err(crate::error::Error::from)
}

pub(crate) async fn get_document_tree_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentTreeRequest,
    access_control: &AccessControl,
) -> Result<GetDocumentTreeResponse> {
    let client_guard = client.lock().await;
    let project_id_or_key_val = ProjectIdOrKey::from_str(req.project_id_or_key.trim())?;

    // Check project access with parsed type
    access_control
        .check_project_access_id_or_key_async(&project_id_or_key_val, &client_guard)
        .await?;
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
