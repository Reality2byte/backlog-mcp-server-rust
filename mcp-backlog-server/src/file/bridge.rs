use crate::file::request::GetSharedFilesListRequest;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::ProjectIdOrKey;
use backlog_file::{models::SharedFile, requests::GetSharedFilesListParams};
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
        order: request.order,
        offset: request.offset,
        count: request.count,
    };

    client_guard
        .file()
        .get_shared_files_list(project_id_or_key, request.path, params)
        .await
        .map_err(|e| {
            McpError::internal_error(format!("Failed to get shared files list: {}", e), None)
        })
}
