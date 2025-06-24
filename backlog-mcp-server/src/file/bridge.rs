use crate::file::request::{DownloadSharedFileRequest, GetSharedFilesListRequest};
use backlog_api_client::{DownloadedFile, client::BacklogApiClient};
use backlog_core::{ProjectIdOrKey, identifier::SharedFileId};
use backlog_file::{GetFileParams, GetSharedFilesListParams, SharedFile};
use rmcp::Error as McpError;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) async fn get_shared_files_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    request: GetSharedFilesListRequest,
) -> Result<Vec<SharedFile>, McpError> {
    let client_guard = client.lock().await;

    let project_id_or_key = ProjectIdOrKey::from_str(&request.project_id_or_key).map_err(|e| {
        McpError::invalid_request(format!("Invalid project ID or key: {}", e), None)
    })?;

    let params = GetSharedFilesListParams {
        project_id_or_key,
        path: request.path,
        order: request.order,
        offset: request.offset,
        count: request.count,
    };

    client_guard
        .file()
        .get_shared_files_list(params)
        .await
        .map_err(|e| {
            McpError::internal_error(format!("Failed to get shared files list: {}", e), None)
        })
}

pub(crate) async fn download_shared_file_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    request: DownloadSharedFileRequest,
) -> Result<DownloadedFile, McpError> {
    let client_guard = client.lock().await;

    let project_id_or_key = ProjectIdOrKey::from_str(&request.project_id_or_key).map_err(|e| {
        McpError::invalid_request(format!("Invalid project ID or key: {}", e), None)
    })?;

    let shared_file_id = SharedFileId::new(request.shared_file_id);
    let params = GetFileParams::new(project_id_or_key, shared_file_id);

    client_guard.file().get_file(params).await.map_err(|e| {
        McpError::internal_error(format!("Failed to download shared file: {}", e), None)
    })
}
