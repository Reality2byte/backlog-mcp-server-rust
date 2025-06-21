#[cfg(feature = "git_writable")]
use backlog_api_client::{AddPullRequestCommentParams, AddPullRequestCommentParamsBuilder};
use backlog_api_client::{
    ApiError, GetPullRequestCommentListParams, GetPullRequestCommentListParamsBuilder,
    PrCommentOrder,
};
#[cfg(feature = "git_writable")]
use backlog_core::identifier::UserId;
use rmcp::schemars::{self, JsonSchema}; // rmcp::schemars を使用
use serde::Deserialize;
use std::str::FromStr;

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
pub struct DownloadPullRequestAttachmentRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    /// The pull request number.
    pub pr_number: u64,
    /// The numeric ID of the attachment to download.
    pub attachment_id: u32,
    /// Optional format specification: 'image', 'text', or 'raw'. If not specified, format will be auto-detected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
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

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GetPullRequestCommentListRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    /// The pull request number.
    pub pr_number: u64,
    /// The minimum comment ID to retrieve.
    #[serde(default)]
    #[schemars(description = "The minimum comment ID to retrieve.")]
    pub min_id: Option<u32>,
    /// The maximum comment ID to retrieve.
    #[serde(default)]
    #[schemars(description = "The maximum comment ID to retrieve.")]
    pub max_id: Option<u32>,
    /// The number of comments to retrieve (1-100).
    #[serde(default)]
    #[schemars(description = "The number of comments to retrieve (1-100).")]
    pub count: Option<u8>,
    /// The sort order: 'asc' or 'desc'.
    #[serde(default)]
    #[schemars(description = "The sort order: 'asc' or 'desc'.")]
    pub order: Option<String>,
}

impl TryFrom<GetPullRequestCommentListRequest> for GetPullRequestCommentListParams {
    type Error = ApiError;

    fn try_from(req: GetPullRequestCommentListRequest) -> Result<Self, Self::Error> {
        let order = req
            .order
            .as_deref()
            .map(PrCommentOrder::from_str)
            .transpose()?;

        GetPullRequestCommentListParamsBuilder::default()
            .min_id(req.min_id)
            .max_id(req.max_id)
            .count(req.count)
            .order(order)
            .build()
    }
}

#[cfg(feature = "git_writable")]
#[derive(Deserialize, JsonSchema, Debug)]
pub struct AddPullRequestCommentRequest {
    /// The project ID or project key.
    pub project_id_or_key: String,
    /// The repository ID (as a string) or repository name.
    pub repo_id_or_name: String,
    /// The pull request number.
    pub pr_number: u64,
    /// The content of the comment.
    pub content: String,
    /// Optional list of user IDs to notify about this comment.
    #[serde(default)]
    #[schemars(description = "Optional list of user IDs to notify about this comment.")]
    pub notified_user_ids: Option<Vec<u32>>,
}

#[cfg(feature = "git_writable")]
impl TryFrom<AddPullRequestCommentRequest> for AddPullRequestCommentParams {
    type Error = ApiError;

    fn try_from(req: AddPullRequestCommentRequest) -> Result<Self, Self::Error> {
        use backlog_api_client::{ProjectIdOrKey, PullRequestNumber, RepositoryIdOrName};
        use std::str::FromStr;

        let project_id_or_key = req.project_id_or_key.parse::<ProjectIdOrKey>()?;
        let repo_id_or_name = RepositoryIdOrName::from_str(req.repo_id_or_name.trim())?;
        let pr_number = PullRequestNumber::from(req.pr_number);

        let notified_user_ids = req
            .notified_user_ids
            .map(|ids| ids.into_iter().map(UserId::new).collect());

        AddPullRequestCommentParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .pr_number(pr_number)
            .content(req.content)
            .notified_user_ids(notified_user_ids)
            .build()
    }
}
