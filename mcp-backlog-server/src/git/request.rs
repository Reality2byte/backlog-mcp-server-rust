use rmcp::schemars::{self, JsonSchema}; // rmcp::schemars を使用
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, Debug)]
pub struct ListPullRequestsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    // TODO: Add other query parameters like statusId[], assigneeId[], etc.
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GetPullRequestDetailsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    /// The pull request number.
    pub pr_number: u64,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GetRepositoryDetailsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GetRepositoryListRequest {
    /// The project ID or project key to retrieve repositories for. Examples: "MYPROJECTKEY", "123".
    pub project_id_or_key: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GetPullRequestAttachmentListRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    /// The pull request number.
    pub pr_number: u64,
}
