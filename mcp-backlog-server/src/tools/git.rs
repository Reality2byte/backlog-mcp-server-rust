//! MCP tools for interacting with Backlog Git repositories and Pull Requests.

use backlog_api_client::client::BacklogApiClient;
use backlog_core::project_id_or_key::ProjectIdOrKey;
use backlog_git::{PullRequest, Repository};
use rmcp::{Error as McpError, schemars, serde};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRepositoryListRequest {
    /// The project ID or project key to retrieve repositories for.
    /// Examples: "MYPROJECTKEY", "123".
    pub project_id_or_key: String,
}

// The response for get_repository_list will be Vec<Repository>.
// The Repository struct from backlog_git already derives Serialize and JsonSchema.

// Tool implementation will be methods on the Server struct in server.rs,
// which will call helper functions defined here or directly use the BacklogApiClient.

/// Helper function to implement the get_repository_list tool.
pub async fn get_repository_list_impl(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    request: GetRepositoryListRequest,
) -> Result<Vec<Repository>, McpError> {
    let project_id = request
        .project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| {
            McpError::invalid_params(
                format!(
                    // Corrected to invalid_params and added None
                    "Invalid project_id_or_key '{}': {}",
                    request.project_id_or_key, e
                ),
                None,
            )
        })?;

    let client_guard = client.lock().await; // Added lock
    client_guard
        .git()
        .list_repositories(&project_id)
        .await
        .map_err(|e| McpError::internal_error(format!("Failed to list repositories: {}", e), None)) // Added None
}

// TODO: Add other request structs and impl functions:
// GetRepositoryDetailsRequest -> Repository
// ListPullRequestsRequest -> Vec<PullRequest>
// GetPullRequestDetailsRequest -> PullRequest

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRepositoryDetailsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
}

pub async fn get_repository_details_impl(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    request: GetRepositoryDetailsRequest,
) -> Result<Repository, McpError> {
    let project_id = request
        .project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| {
            McpError::invalid_params(
                format!(
                    // Corrected to invalid_params and added None
                    "Invalid project_id_or_key '{}': {}",
                    request.project_id_or_key, e
                ),
                None,
            )
        })?;
    let client_guard = client.lock().await; // Added lock
    client_guard
        .git()
        .get_repository(&project_id, &request.repo_id_or_name)
        .await
        .map_err(|e| {
            McpError::internal_error(format!("Failed to get repository details: {}", e), None)
        }) // Added None
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListPullRequestsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    // TODO: Add query parameters like status, assignee, etc.
}

pub async fn list_pull_requests_impl(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    request: ListPullRequestsRequest,
) -> Result<Vec<PullRequest>, McpError> {
    let project_id = request
        .project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| {
            McpError::invalid_params(
                format!(
                    // Corrected to invalid_params and added None
                    "Invalid project_id_or_key '{}': {}",
                    request.project_id_or_key, e
                ),
                None,
            )
        })?;
    let client_guard = client.lock().await; // Added lock
    client_guard
        .git()
        .list_pull_requests(&project_id, &request.repo_id_or_name)
        .await
        .map_err(|e| McpError::internal_error(format!("Failed to list pull requests: {}", e), None)) // Added None
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetPullRequestDetailsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    /// The pull request number.
    pub pr_number: u64,
}

pub async fn get_pull_request_details_impl(
    client: Arc<Mutex<BacklogApiClient>>, // Changed signature
    request: GetPullRequestDetailsRequest,
) -> Result<PullRequest, McpError> {
    let project_id = request
        .project_id_or_key
        .parse::<ProjectIdOrKey>()
        .map_err(|e| {
            McpError::invalid_params(
                format!(
                    // Corrected to invalid_params and added None
                    "Invalid project_id_or_key '{}': {}",
                    request.project_id_or_key, e
                ),
                None,
            )
        })?;
    let client_guard = client.lock().await; // Added lock
    client_guard
        .git()
        .get_pull_request(&project_id, &request.repo_id_or_name, request.pr_number)
        .await
        .map_err(|e| {
            McpError::internal_error(format!("Failed to get pull request details: {}", e), None)
        }) // Added None
}
