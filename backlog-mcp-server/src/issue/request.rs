use backlog_api_client::{
    AddCommentParams, AddCommentParamsBuilder, ApiError, CommentOrder, GetCommentListParams,
    GetCommentListParamsBuilder, IssueIdOrKey, UpdateIssueParams, UpdateIssueParamsBuilder,
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

#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
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
    #[schemars(
        description = "Optional custom fields as a JSON object mapping field names to values. Use get_custom_field_list to see available fields and their expected formats."
    )]
    pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl TryFrom<UpdateIssueRequest> for UpdateIssueParams {
    type Error = ApiError;
    fn try_from(req: UpdateIssueRequest) -> Result<Self, Self::Error> {
        use backlog_api_client::IssueIdOrKey;
        use std::str::FromStr;

        let issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
        let mut builder = UpdateIssueParamsBuilder::default();
        builder.issue_id_or_key(issue_id_or_key);
        if let Some(summary) = req.summary {
            builder.summary(summary);
        }
        if let Some(description) = req.description {
            builder.description(description);
        }
        builder.build()
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetAttachmentListRequest {
    #[schemars(
        description = "The issue ID or issue key for which to retrieve attachments. Examples: 'MYPROJECTKEY-123', '12345'."
    )]
    pub issue_id_or_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct DownloadAttachmentRequest {
    #[schemars(description = "The issue ID or issue key. Examples: 'MYPROJECTKEY-123', '12345'.")]
    pub issue_id_or_key: String,
    #[schemars(description = "The numeric ID of the attachment to download.")]
    pub attachment_id: u32,
    #[schemars(
        description = "Optional format specification: 'image', 'text', or 'raw'. If not specified, format will be auto-detected."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
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

impl TryFrom<GetIssueCommentsRequest> for GetCommentListParams {
    type Error = ApiError;
    fn try_from(req: GetIssueCommentsRequest) -> Result<Self, Self::Error> {
        let issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
        let mut params_builder = GetCommentListParamsBuilder::default();
        params_builder.issue_id_or_key(issue_id_or_key);

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
        params_builder.build()
    }
}

#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct AddCommentRequest {
    #[schemars(
        description = "The issue ID or issue key to add the comment to. Examples: 'MYPROJECTKEY-123', '12345'."
    )]
    pub issue_id_or_key: String,
    #[schemars(description = "The content of the comment to add.")]
    pub content: String,
    #[schemars(description = "User IDs to notify about this comment (optional).")]
    pub notified_user_ids: Option<Vec<u32>>,
    #[schemars(description = "Attachment IDs to include with this comment (optional).")]
    pub attachment_ids: Option<Vec<u32>>,
}

impl TryFrom<AddCommentRequest> for AddCommentParams {
    type Error = ApiError;
    fn try_from(req: AddCommentRequest) -> Result<Self, Self::Error> {
        use backlog_api_client::{AttachmentId, IssueIdOrKey, UserId};
        use std::str::FromStr;

        let issue_id_or_key = IssueIdOrKey::from_str(req.issue_id_or_key.trim())?;
        let mut builder = AddCommentParamsBuilder::default();
        builder.issue_id_or_key(issue_id_or_key);
        builder.content(req.content);

        if let Some(user_ids) = req.notified_user_ids {
            let parsed_user_ids: Vec<UserId> = user_ids.into_iter().map(UserId::new).collect();
            builder.notified_user_id(parsed_user_ids);
        }

        if let Some(attachment_ids) = req.attachment_ids {
            let parsed_attachment_ids: Vec<AttachmentId> =
                attachment_ids.into_iter().map(AttachmentId::new).collect();
            builder.attachment_id(parsed_attachment_ids);
        }

        builder.build()
    }
}

#[cfg(feature = "issue_writable")]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct UpdateCommentRequest {
    #[schemars(
        description = "The issue ID or issue key containing the comment. Examples: 'MYPROJECTKEY-123', '12345'."
    )]
    pub issue_id_or_key: String,
    #[schemars(description = "The ID of the comment to update.")]
    pub comment_id: u32,
    #[schemars(description = "The new content for the comment.")]
    pub content: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetIssueSharedFilesRequest {
    #[schemars(
        description = "The issue ID or issue key for which to retrieve shared files. Examples: 'MYPROJECTKEY-123', '12345'."
    )]
    pub issue_id_or_key: String,
}

#[cfg(feature = "issue_writable")]
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct AddIssueRequest {
    #[schemars(
        description = "The project ID or project key to create the issue in. Examples: 'MYPROJECTKEY', '123'."
    )]
    pub project_id: String,
    #[schemars(description = "The title/summary of the issue.")]
    pub summary: String,
    #[schemars(
        description = "The ID of the issue type. get_project_issue_types tool can be used to retrieve valid IDs."
    )]
    pub issue_type_id: u32,
    #[schemars(
        description = "The ID of the priority. get_priorities tool can be used to retrieve valid IDs."
    )]
    pub priority_id: u32,
    #[schemars(description = "Optional description of the issue.")]
    pub description: Option<String>,
    #[schemars(
        description = "Optional custom fields as a JSON object mapping field names to values. Use get_custom_field_list to see available fields and their expected formats."
    )]
    pub custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
