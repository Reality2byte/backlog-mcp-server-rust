use crate::file_utils::{FileFormat, SerializableFile};
#[cfg(feature = "issue_writable")]
use crate::issue::request::UpdateCommentRequest;
use crate::issue::request::{
    GetIssueCommentsRequest, GetIssueSharedFilesRequest, UpdateIssueRequest,
};
use crate::{
    document::{
        self,
        request::{
            DownloadDocumentAttachmentRequest, GetDocumentDetailsRequest, GetDocumentTreeRequest,
        },
    },
    file::{
        self,
        request::{DownloadSharedFileRequest, GetSharedFilesListRequest},
    },
    git::{
        self,
        request::{
            DownloadPullRequestAttachmentRequest, GetPullRequestAttachmentListRequest,
            GetPullRequestCommentListRequest, GetPullRequestDetailsRequest,
            GetRepositoryDetailsRequest, GetRepositoryListRequest, ListPullRequestsRequest,
        },
    },
    issue::{
        self,
        request::{
            AddCommentRequest, DownloadAttachmentRequest, GetAttachmentListRequest,
            GetIssueDetailsRequest, GetIssuesByMilestoneNameRequest,
            GetVersionMilestoneListRequest,
        },
    },
    project::{self, request::GetProjectStatusListRequest},
    user::{self, request::GetUserListRequest},
    wiki::{
        self,
        request::{
            DownloadWikiAttachmentRequest, GetWikiAttachmentListRequest, GetWikiDetailRequest,
            GetWikiListRequest,
        },
    },
};

#[cfg(feature = "wiki_writable")]
use crate::wiki::request::UpdateWikiRequest;

#[cfg(feature = "git_writable")]
use crate::git::request::AddPullRequestCommentRequest;
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
        let repositories = git::bridge::get_repository_list(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(repositories)?]))
    }

    #[tool(description = "Get details for a specific Git repository.")]
    async fn get_repository(
        &self,
        #[tool(aggr)] request: GetRepositoryDetailsRequest,
    ) -> McpResult {
        let repository = git::bridge::get_repository(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(repository)?]))
    }

    #[tool(description = "Get a list of pull requests for a specified repository.")]
    async fn get_pull_request_list(
        &self,
        #[tool(aggr)] request: ListPullRequestsRequest,
    ) -> McpResult {
        let pull_requests =
            git::bridge::get_pull_request_list(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(pull_requests)?]))
    }

    #[tool(description = "Get details for a specific pull request.")]
    async fn get_pull_request(
        &self,
        #[tool(aggr)] request: GetPullRequestDetailsRequest,
    ) -> McpResult {
        let pull_request = git::bridge::get_pull_request(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(pull_request)?]))
    }

    #[tool(description = "Get details for a specific Backlog issue.")]
    async fn get_issue(&self, #[tool(aggr)] request: GetIssueDetailsRequest) -> McpResult {
        let issue = issue::bridge::get_issue_details(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(issue)?]))
    }

    #[tool(description = "Get details for a specific Backlog document.
     This API returns the document details including its title, `plain` as Markdown and `json` as ProseMirror json, and other metadata.")]
    async fn get_document_details(
        &self,
        #[tool(aggr)] request: GetDocumentDetailsRequest,
    ) -> McpResult {
        let document = document::bridge::get_document_details(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(document)?]))
    }

    #[tool(
        description = "Download a document attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn download_document_attachment(
        &self,
        #[tool(aggr)] request: DownloadDocumentAttachmentRequest,
    ) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file =
            document::bridge::download_document_attachment_bridge(self.client.clone(), request)
                .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get the document tree for a specified project.")]
    async fn get_document_tree(&self, #[tool(aggr)] request: GetDocumentTreeRequest) -> McpResult {
        let document_tree =
            document::bridge::get_document_tree_tool(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(
            document_tree.active_tree,
        )?]))
    }

    #[tool(description = "Get a list of versions (milestones) for a specified project.")]
    async fn get_version_milestone_list(
        &self,
        #[tool(aggr)] request: GetVersionMilestoneListRequest,
    ) -> McpResult {
        let milestones =
            issue::bridge::get_version_milestone_list(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(milestones)?]))
    }

    #[tool(description = "Get a list of issues for a specified milestone name within a project.")]
    async fn get_issues_by_milestone_name(
        &self,
        #[tool(aggr)] request: GetIssuesByMilestoneNameRequest,
    ) -> McpResult {
        let issues =
            issue::bridge::get_issues_by_milestone_name(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(issues)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Update the summary and/or description of a Backlog issue.")]
    async fn update_issue(&self, #[tool(aggr)] request: UpdateIssueRequest) -> McpResult {
        let updated_issue = issue::bridge::update_issue_impl(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(updated_issue)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Add a comment to a Backlog issue.")]
    async fn add_comment_to_issue(&self, #[tool(aggr)] request: AddCommentRequest) -> McpResult {
        let comment = issue::bridge::add_comment_impl(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Update an existing comment on a Backlog issue.")]
    async fn update_issue_comment(&self, #[tool(aggr)] request: UpdateCommentRequest) -> McpResult {
        let comment = issue::bridge::update_comment_impl(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }

    #[tool(
        name = "get_issue_comments",
        description = "Gets comments for a specific issue. Takes 'issue_id_or_key' (string, required) and optional 'min_id', 'max_id', 'count', 'order' parameters."
    )]
    async fn get_issue_comments(
        &self,
        #[tool(aggr)] request: GetIssueCommentsRequest,
    ) -> McpResult {
        let comments = issue::bridge::get_issue_comments_impl(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(comments)?]))
    }

    #[tool(
        name = "get_issue_attachment_list",
        description = "Get a list of attachments for a specified issue."
    )]
    async fn get_issue_attachment_list(
        &self,
        #[tool(aggr)] request: GetAttachmentListRequest,
    ) -> McpResult {
        let attachments =
            issue::bridge::get_attachment_list_impl(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        name = "get_issue_shared_files",
        description = "Get a list of shared files linked to a specified issue."
    )]
    async fn get_issue_shared_files(
        &self,
        #[tool(aggr)] request: GetIssueSharedFilesRequest,
    ) -> McpResult {
        let shared_files =
            issue::bridge::get_issue_shared_files_impl(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(shared_files)?]))
    }

    #[tool(description = "Get a list of users in the space.")]
    async fn get_user_list(&self, #[tool(aggr)] request: GetUserListRequest) -> McpResult {
        let users = user::bridge::get_user_list_bridge(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(users)?]))
    }

    #[tool(
        description = "Download an issue attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn download_issue_attachment(
        &self,
        #[tool(aggr)] request: DownloadAttachmentRequest,
    ) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file =
            issue::bridge::download_issue_attachment_file(self.client.clone(), request).await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get a list of statuses for a specified project.")]
    async fn get_project_status_list(
        &self,
        #[tool(aggr)] request: GetProjectStatusListRequest,
    ) -> McpResult {
        let statuses =
            project::bridge::get_project_status_list_tool(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(statuses)?]))
    }

    #[tool(description = "Get a list of shared files for a specified project directory.")]
    async fn get_shared_files_list(
        &self,
        #[tool(aggr)] request: GetSharedFilesListRequest,
    ) -> McpResult {
        let files = file::bridge::get_shared_files_list_tool(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(files)?]))
    }

    #[tool(
        description = "Download a shared file. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn download_shared_file(
        &self,
        #[tool(aggr)] request: DownloadSharedFileRequest,
    ) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = file::bridge::download_shared_file_bridge(self.client.clone(), request).await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get a list of attachments for a specific pull request.")]
    async fn get_pull_request_attachment_list(
        &self,
        #[tool(aggr)] request: GetPullRequestAttachmentListRequest,
    ) -> McpResult {
        let attachments =
            git::bridge::get_pull_request_attachment_list_tool(self.client.clone(), request)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        description = "Download a pull request attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn download_pull_request_attachment(
        &self,
        #[tool(aggr)] request: DownloadPullRequestAttachmentRequest,
    ) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = git::bridge::download_pr_attachment_bridge(self.client.clone(), request).await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get a list of comments for a specific pull request.")]
    async fn get_pull_request_comment_list(
        &self,
        #[tool(aggr)] request: GetPullRequestCommentListRequest,
    ) -> McpResult {
        let comments =
            git::bridge::get_pull_request_comment_list_tool(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(comments)?]))
    }

    #[tool(description = "Get detailed information about a specific wiki page.")]
    async fn get_wiki_detail(&self, #[tool(aggr)] request: GetWikiDetailRequest) -> McpResult {
        let client = self.client.lock().await;
        let detail = wiki::bridge::get_wiki_detail(&client, request).await?;
        Ok(CallToolResult::success(vec![Content::json(detail)?]))
    }

    #[tool(description = "Get a list of wiki pages. Can be filtered by project and keyword.")]
    async fn get_wiki_list(&self, #[tool(aggr)] request: GetWikiListRequest) -> McpResult {
        let client = self.client.lock().await;
        let wikis = wiki::bridge::get_wiki_list(&client, request).await?;
        Ok(CallToolResult::success(vec![Content::json(wikis)?]))
    }

    #[tool(description = "Get a list of attachments for a specified wiki page.")]
    async fn get_wiki_attachment_list(
        &self,
        #[tool(aggr)] request: GetWikiAttachmentListRequest,
    ) -> McpResult {
        let client = self.client.lock().await;
        let attachments = wiki::bridge::get_wiki_attachment_list(&client, request).await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        description = "Download an attachment from a wiki page. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn download_wiki_attachment(
        &self,
        #[tool(aggr)] request: DownloadWikiAttachmentRequest,
    ) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let client = self.client.lock().await;
        let file = wiki::bridge::download_wiki_attachment(&client, request).await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[cfg(feature = "wiki_writable")]
    #[tool(
        description = "Update a wiki page. You can update the page name, content, and/or email notification settings."
    )]
    async fn update_wiki(&self, #[tool(aggr)] request: UpdateWikiRequest) -> McpResult {
        let client = self.client.lock().await;
        let wiki_detail = wiki::bridge::update_wiki(&client, request).await?;
        Ok(CallToolResult::success(vec![Content::json(wiki_detail)?]))
    }

    #[cfg(feature = "git_writable")]
    #[tool(
        description = "Add a comment to a specific pull request. Optionally notify specified users."
    )]
    async fn add_pull_request_comment(
        &self,
        #[tool(aggr)] request: AddPullRequestCommentRequest,
    ) -> McpResult {
        let comment =
            git::bridge::add_pull_request_comment_bridge(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
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
