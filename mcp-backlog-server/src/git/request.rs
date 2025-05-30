use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRepositoryListRequest {
    /// The project ID or project key to retrieve repositories for.
    /// Examples: "MYPROJECTKEY", "123".
    pub project_id_or_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRepositoryDetailsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListPullRequestsRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    // TODO: Add query parameters like status, assignee, etc.
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
