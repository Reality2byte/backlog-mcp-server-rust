use rmcp::schemars;
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
// #[serde(rename_all = "camelCase")] // Removed
pub struct GetProjectStatusListRequest {
    /// The project ID or project key to retrieve statuses for.
    /// Examples: "MYPROJECTKEY", "123".
    /// Ensure there are no leading or trailing spaces.
    pub project_id_or_key: String,
}
