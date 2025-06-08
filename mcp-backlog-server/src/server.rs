use crate::SerializableRawAttachment;
use crate::issue::request::{GetIssueCommentsRequest, UpdateIssueRequest};
use crate::{
    document::{
        self,
        request::{
            DownloadDocumentAttachmentRequest, GetDocumentDetailsRequest, GetDocumentTreeRequest,
        },
    },
    git::{
        self,
        request::{
            DownloadPullRequestAttachmentRequest, GetPullRequestAttachmentListRequest,
            GetPullRequestDetailsRequest, GetRepositoryDetailsRequest, GetRepositoryListRequest,
            ListPullRequestsRequest,
        },
    },
    issue::{
        self,
        request::{
            DownloadAttachmentRequest, GetAttachmentListRequest, GetIssueDetailsRequest,
            GetIssuesByMilestoneNameRequest, GetVersionMilestoneListRequest,
        },
    },
    project::{self, request::GetProjectStatusListRequest},
    user::{self, request::GetUserListRequest},
};
use backlog_api_client::client::BacklogApiClient;
use rmcp::{
    Error as McpError,
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    tool,
};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Server {
    client: Arc<Mutex<BacklogApiClient>>,
}

type McpResult = Result<CallToolResult, McpError>;

#[tool(tool_box)]
impl Server {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = env::var("BACKLOG_BASE_URL")?;
        let api_key = env::var("BACKLOG_API_KEY")?;
        let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    #[tool(description = "Get a list of Git repositories for a specified project.")]
    async fn get_repository_list(
        &self,
        #[tool(aggr)] request: GetRepositoryListRequest,
    ) -> McpResult {
        let repositories =
            git::bridge::get_repository_list(self.client.clone(), request.project_id_or_key)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(repositories)?]))
    }

    #[tool(description = "Get details for a specific Git repository.")]
    async fn get_repository(
        &self,
        #[tool(aggr)] request: GetRepositoryDetailsRequest,
    ) -> McpResult {
        let repository = git::bridge::get_repository(
            self.client.clone(),
            request.project_id_or_key,
            request.repo_id_or_name,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(repository)?]))
    }

    #[tool(description = "Get a list of pull requests for a specified repository.")]
    async fn get_pull_request_list(
        &self,
        #[tool(aggr)] request: ListPullRequestsRequest,
    ) -> McpResult {
        let pull_requests = git::bridge::get_pull_request_list(
            self.client.clone(),
            request.project_id_or_key,
            request.repo_id_or_name,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(pull_requests)?]))
    }

    #[tool(description = "Get details for a specific pull request.")]
    async fn get_pull_request(
        &self,
        #[tool(aggr)] request: GetPullRequestDetailsRequest,
    ) -> McpResult {
        let pull_request = git::bridge::get_pull_request(
            self.client.clone(),
            request.project_id_or_key,
            request.repo_id_or_name,
            request.pr_number,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(pull_request)?]))
    }

    #[tool(description = "Get details for a specific Backlog issue.")]
    async fn get_issue(&self, #[tool(aggr)] req: GetIssueDetailsRequest) -> McpResult {
        let issue = issue::bridge::get_issue_details(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(issue)?]))
    }

    #[tool(description = "Get details for a specific Backlog document.
     This API returns the document details including its title, `plain` as Markdown and `json` as ProseMirror json, and other metadata.")]
    async fn get_document_details(
        &self,
        #[tool(aggr)] req: GetDocumentDetailsRequest,
    ) -> McpResult {
        let document = document::bridge::get_document_details(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(document)?]))
    }

    #[tool(
        description = "Download a document attachment if it is an image. Returns image content."
    )]
    async fn download_document_attachment_image(
        &self,
        #[tool(aggr)] req: DownloadDocumentAttachmentRequest,
    ) -> McpResult {
        let file =
            document::bridge::download_document_attachment_bridge(self.client.clone(), req).await?;
        let response_data = SerializableRawAttachment::image(file)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get the document tree for a specified project.")]
    async fn get_document_tree(&self, #[tool(aggr)] req: GetDocumentTreeRequest) -> McpResult {
        let document_tree =
            document::bridge::get_document_tree_tool(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(document_tree.active_tree)?]))
    }

    #[tool(description = "Get a list of versions (milestones) for a specified project.")]
    async fn get_version_milestone_list(
        &self,
        #[tool(aggr)] req: GetVersionMilestoneListRequest,
    ) -> McpResult {
        let milestones =
            issue::bridge::get_version_milestone_list(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(milestones)?]))
    }

    #[tool(description = "Get a list of issues for a specified milestone name within a project.")]
    async fn get_issues_by_milestone_name(
        &self,
        #[tool(aggr)] req: GetIssuesByMilestoneNameRequest,
    ) -> McpResult {
        let issues = issue::bridge::get_issues_by_milestone_name(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(issues)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Update the summary and/or description of a Backlog issue.")]
    async fn update_issue(&self, #[tool(aggr)] req: UpdateIssueRequest) -> McpResult {
        let updated_issue = issue::bridge::update_issue_impl(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(updated_issue)?]))
    }

    #[tool(
        name = "get_issue_comments",
        description = "Gets comments for a specific issue. Takes 'issue_id_or_key' (string, required) and optional 'min_id', 'max_id', 'count', 'order' parameters."
    )]
    async fn get_issue_comments(&self, #[tool(aggr)] req: GetIssueCommentsRequest) -> McpResult {
        let comments = issue::bridge::get_issue_comments_impl(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(comments)?]))
    }

    #[tool(
        name = "get_issue_attachment_list",
        description = "Get a list of attachments for a specified issue."
    )]
    async fn get_issue_attachment_list(
        &self,
        #[tool(aggr)] req: GetAttachmentListRequest,
    ) -> McpResult {
        let attachments = issue::bridge::get_attachment_list_impl(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(description = "Get a list of users in the space.")]
    async fn get_user_list(&self, #[tool(aggr)] req: GetUserListRequest) -> McpResult {
        let users = user::bridge::get_user_list_bridge(req, self.client.clone()).await?;
        Ok(CallToolResult::success(vec![Content::json(users)?]))
    }

    #[tool(
        description = "Download an issue attachment as raw bytes. Returns a JSON object with filename, MIME type, and base64-encoded content."
    )]
    async fn download_issue_attachment_raw(
        &self,
        #[tool(aggr)] req: DownloadAttachmentRequest,
    ) -> McpResult {
        let file = issue::bridge::download_issue_attachment_file(self.client.clone(), req).await?;
        let response_data = SerializableRawAttachment::raw(file);
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Download an issue attachment image. Returns filename and image content.")]
    async fn download_issue_attachment_image(
        &self,
        #[tool(aggr)] req: DownloadAttachmentRequest,
    ) -> McpResult {
        let file = issue::bridge::download_issue_attachment_file(self.client.clone(), req).await?;
        let response_data = SerializableRawAttachment::image(file)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(
        description = "Download an issue attachment if it is a valid UTF-8 text file. Returns the text content."
    )]
    async fn download_issue_attachment_text(
        &self,
        #[tool(aggr)] req: DownloadAttachmentRequest,
    ) -> McpResult {
        let file = issue::bridge::download_issue_attachment_file(self.client.clone(), req).await?;
        let response_data = SerializableRawAttachment::text(file)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get a list of statuses for a specified project.")]
    async fn get_project_status_list(
        &self,
        #[tool(aggr)] req: GetProjectStatusListRequest,
    ) -> McpResult {
        let statuses =
            project::bridge::get_project_status_list_tool(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(statuses)?]))
    }

    #[tool(description = "Get a list of attachments for a specific pull request.")]
    async fn get_pull_request_attachment_list(
        &self,
        #[tool(aggr)] req: GetPullRequestAttachmentListRequest,
    ) -> McpResult {
        let attachments =
            git::bridge::get_pull_request_attachment_list_tool(self.client.clone(), req).await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        description = "Download a pull request attachment as raw bytes. Returns filename and raw byte content."
    )]
    async fn download_pull_request_attachment_raw(
        &self,
        #[tool(aggr)] req: DownloadPullRequestAttachmentRequest,
    ) -> McpResult {
        let file = git::bridge::download_pr_attachment_bridge(self.client.clone(), req).await?;
        let response_data = SerializableRawAttachment::raw(file);
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(
        description = "Download a pull request attachment image. Returns filename and image content as base64."
    )]
    async fn download_pull_request_attachment_image(
        &self,
        #[tool(aggr)] req: DownloadPullRequestAttachmentRequest,
    ) -> McpResult {
        let file = git::bridge::download_pr_attachment_bridge(self.client.clone(), req).await?;
        let response_data = SerializableRawAttachment::image(file)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(
        description = "Download a pull request attachment if it is a valid UTF-8 text file. Returns the text content."
    )]
    async fn download_pull_request_attachment_text(
        &self,
        #[tool(aggr)] req: DownloadPullRequestAttachmentRequest,
    ) -> McpResult {
        let file = git::bridge::download_pr_attachment_bridge(self.client.clone(), req).await?;
        let response_data = SerializableRawAttachment::text(file)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }
}

#[tool(tool_box)]
impl rmcp::ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        let instructions = "Backlog MCP Server\n\n\
This server provides tools to interact with Backlog, a project management service.
"
        .to_string();
        ServerInfo {
            instructions: Some(instructions),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
