use backlog_api_client::{
    CommentOrder, CoreError, GetCommentListParamsBuilder, UpdateIssueParamsBuilder,
};
use rmcp::schemars;
use std::str::FromStr;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetIssueDetailsRequest {
    #[schemars(
        description = "The issue key to retrieve details for. \n    This should be in the format 'PROJECT-123', where 'PROJECT' is the project key and '123' is the issue number. \n    Ensure there are no leading or trailing spaces."
    )]
    pub issue_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetVersionMilestoneListRequest {
    #[schemars(
        description = "The project ID or project key to retrieve versions (milestones) for. \n    Examples: 'MYPROJECTKEY', '123'. \n    Ensure there are no leading or trailing spaces."
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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetAttachmentListRequest {
    #[schemars(
        description = "The issue ID or issue key for which to retrieve attachments. Examples: 'MYPROJECTKEY-123', '12345'."
    )]
    pub issue_id_or_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DownloadAttachmentRequest {
    #[schemars(description = "The issue ID or issue key. Examples: 'MYPROJECTKEY-123', '12345'.")]
    pub issue_id_or_key: String, // Corrected to match serde rename_all
    #[schemars(description = "The numeric ID of the attachment to download.")]
    pub attachment_id: u32, // Corrected to match serde rename_all
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetIssueCommentsRequest {
    #[schemars(description = "Issue ID or key (e.g., 'PROJECT-123').")]
    pub issue_id_or_key: String,
    #[schemars(description = "Min comment ID.")]
    pub min_id: Option<u64>,
    #[schemars(description = "Max comment ID.")]
    pub max_id: Option<u64>,
    #[schemars(description = "Number of comments to retrieve (1-100).")]
    pub count: Option<u8>,
    #[schemars(description = "Sort order: 'asc' or 'desc'.")]
    pub order: Option<String>,
}

impl TryFrom<GetIssueCommentsRequest> for GetCommentListParamsBuilder {
    type Error = CoreError;
    fn try_from(req: GetIssueCommentsRequest) -> Result<Self, Self::Error> {
        let mut params_builder = GetCommentListParamsBuilder::default();

        if let Some(min_id) = req.min_id {
            params_builder.min_id(min_id);
        }
        if let Some(max_id) = req.max_id {
            params_builder.max_id(max_id);
        }
        if let Some(count) = req.count {
            params_builder.count(count);
        }
        let parsed_order: Option<CommentOrder> = req
            .order
            .as_deref()
            .map(CommentOrder::from_str)
            .transpose()?;
        if let Some(order) = parsed_order {
            params_builder.order(order);
        }
        Ok(params_builder)
    }
}
