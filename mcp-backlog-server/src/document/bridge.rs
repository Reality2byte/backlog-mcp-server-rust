use backlog_api_client::client::BacklogApiClient;
use backlog_api_client::{ApiError, DocumentDetail, DocumentId};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::request::GetDocumentDetailsRequest;
use crate::error::Result;

pub async fn get_document_details(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetDocumentDetailsRequest,
) -> Result<DocumentDetail> {
    let client = client.lock().await;
    let document_id = DocumentId::from_str(req.document_id.trim()).map_err(ApiError::from)?;
    let document = client.document().get_document(document_id.clone()).await?;
    Ok(document)
}
