use rmcp::schemars;
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetProjectStatusListRequest {
    /// The project ID or project key to retrieve statuses for.
    /// Examples: "MYPROJECTKEY", "123".
    /// Ensure there are no leading or trailing spaces.
    pub project_id_or_key: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetProjectIssueTypesRequest {
    /// The project ID or project key to retrieve issue types for.
    /// Examples: "MYPROJECTKEY", "123".
    /// Ensure there are no leading or trailing spaces.
    pub project_id_or_key: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetPrioritiesRequest {
    // No parameters needed for priorities - they are global
}
