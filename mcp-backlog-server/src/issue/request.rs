use backlog_api_client::UpdateIssueParamsBuilder;
use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetIssueDetailsRequest {
    #[schemars(description = "The issue key to retrieve details for. 
    This should be in the format 'PROJECT-123', where 'PROJECT' is the project key and '123' is the issue number. 
    Ensure there are no leading or trailing spaces.")]
    pub issue_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetVersionMilestoneListRequest {
    #[schemars(
        description = "The project ID or project key to retrieve versions (milestones) for. 
    Examples: 'MYPROJECTKEY', '123'. 
    Ensure there are no leading or trailing spaces."
    )]
    pub project_id_or_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetIssuesByMilestoneNameRequest {
    #[schemars(
        description = "The project ID or project key where the milestone belongs. Examples: 'MYPROJECTKEY', '123'."
    )]
    pub project_id_or_key: String,
    #[schemars(description = "The name of the milestone to retrieve issues for.")]
    pub milestone_name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct UpdateIssueRequest {
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

impl From<UpdateIssueRequest> for UpdateIssueParamsBuilder {
    fn from(req: UpdateIssueRequest) -> Self {
        let mut builder = UpdateIssueParamsBuilder::default();
        if let Some(summary) = req.summary {
            builder.summary(summary);
        }
        if let Some(description) = req.description {
            builder.description(description);
        }
        builder
    }
}
