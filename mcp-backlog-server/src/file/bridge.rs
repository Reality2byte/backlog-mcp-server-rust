use crate::error::Result;
use crate::file::request::GetSharedFilesListRequest;
use backlog_api_client::ProjectIdOrKey;
use backlog_api_client::client::BacklogApiClient;
use backlog_file::{GetSharedFilesListParams, SharedFile};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Helper function to implement the get_shared_files_list tool.
pub(crate) async fn get_shared_files_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetSharedFilesListRequest,
) -> Result<Vec<SharedFile>> {
    // Parse the project_id_or_key from the request string.
    let project_id = req.project_id_or_key.parse::<ProjectIdOrKey>()?;

    // Convert request parameters to the API parameters struct.
    let params = GetSharedFilesListParams {
        order: req.order,
        offset: req.offset,
        count: req.count,
    };

    let client_guard = client.lock().await;
    // Call the API method with the parsed parameters.
    let files = client_guard
        .file()
        .get_shared_files_list(project_id, &req.path, params)
        .await?;
    Ok(files)
}
