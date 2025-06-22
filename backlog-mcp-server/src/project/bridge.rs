use crate::error::Result;
use crate::project::request::GetProjectStatusListRequest;
use backlog_api_client::ProjectIdOrKey; // From backlog-core, re-exported by backlog-api-client
use backlog_api_client::client::BacklogApiClient;
use backlog_project::Status; // Specific model from backlog-project
use std::sync::Arc;
use tokio::sync::Mutex;

/// Helper function to implement the get_project_status_list tool.
pub(crate) async fn get_project_status_list_tool(
    client: Arc<Mutex<BacklogApiClient>>,
    req: GetProjectStatusListRequest,
) -> Result<Vec<Status>> {
    // Parse the project_id_or_key from the request string.
    // This will use From<CoreError> for Error if parsing fails.
    let project_id = req.project_id_or_key.parse::<ProjectIdOrKey>()?;

    let client_guard = client.lock().await;
    // This will use From<ApiError> for Error if the API call fails.
    let params = backlog_project::GetStatusListParams::new(project_id);
    let statuses = client_guard.project().get_status_list(params).await?;
    Ok(statuses)
}
