use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetIssueDetailsRequest {
    #[schemars(description = "The issue key to retrieve details for. 
    This should be in the format 'PROJECT-123', where 'PROJECT' is the project key and '123' is the issue number. 
    Ensure there are no leading or trailing spaces.")]
    pub issue_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetVersionMilestoneListRequest {
    #[schemars(
        description = "The project ID or project key to retrieve versions (milestones) for. 
    Examples: 'MYPROJECTKEY', '123'. 
    Ensure there are no leading or trailing spaces."
    )]
    pub project_id_or_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetIssuesByMilestoneNameRequest {
    #[schemars(
        description = "The project ID or project key where the milestone belongs. Examples: 'MYPROJECTKEY', '123'."
    )]
    pub project_id_or_key: String,
    #[schemars(description = "The name of the milestone to retrieve issues for.")]
    pub milestone_name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateIssueRequest {
    #[schemars(
        description = "The issue ID or issue key to update. Example: 'MYPROJECTKEY-123' or '12345'."
    )]
    pub issue_id_or_key: String,
    #[schemars(
        description = "The new summary for the issue. Set to null or omit to keep unchanged."
    )]
    pub summary: Option<String>,
    #[schemars(
        description = "The new description for the issue. Set to null or omit to keep unchanged."
    )]
    pub description: Option<String>,
}
