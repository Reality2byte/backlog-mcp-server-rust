use backlog_api_client::DocumentDetail;
use backlog_api_client::DocumentId;
use backlog_api_client::client::BacklogApiClient;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::Result;

pub async fn get_document_details(
    client: Arc<Mutex<BacklogApiClient>>,
    document_id: String,
) -> Result<DocumentDetail> {
    let client = client.lock().await;
    let document_id = DocumentId::from_str(document_id.trim())?;
    let document = client.document().get_document(document_id.clone()).await?;
    Ok(document)
}
