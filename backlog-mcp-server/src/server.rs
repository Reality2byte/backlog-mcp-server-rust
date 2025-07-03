#![allow(unused_imports, dead_code)]

use crate::file_utils::{FileFormat, SerializableFile};
#[cfg(feature = "issue_writable")]
use crate::issue::request::{AddIssueRequest, UpdateCommentRequest};
use crate::issue::request::{
    GetIssueCommentsRequest, GetIssueSharedFilesRequest, UpdateIssueRequest,
};
use crate::issue::response_transformer::IssueResponse;
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
    project::{
        self,
        request::{
            GetCustomFieldListRequest, GetPrioritiesRequest, GetProjectIssueTypesRequest,
            GetProjectStatusListRequest,
        },
    },
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

use crate::access_control::AccessControl;
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
    access_control: AccessControl,
}

type McpResult = Result<CallToolResult, McpError>;

impl Server {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = env::var("BACKLOG_BASE_URL")?;
        let api_key = env::var("BACKLOG_API_KEY")?;
        let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);
        let access_control = AccessControl::new()?;
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            access_control,
        })
    }

    #[tool(description = "Get a list of Git repositories for a specified project.")]
    async fn git_repository_list_get(&self, request: GetRepositoryListRequest) -> McpResult {
        let repositories =
            git::bridge::get_repository_list(self.client.clone(), request, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(repositories)?]))
    }

    #[tool(description = "Get details for a specific Git repository.")]
    async fn git_repository_details_get(&self, request: GetRepositoryDetailsRequest) -> McpResult {
        let repository =
            git::bridge::get_repository(self.client.clone(), request, &self.access_control).await?;
        Ok(CallToolResult::success(vec![Content::json(repository)?]))
    }

    #[tool(description = "Get a list of pull requests for a specified repository.")]
    async fn git_pr_list_get(&self, request: ListPullRequestsRequest) -> McpResult {
        let pull_requests =
            git::bridge::get_pull_request_list(self.client.clone(), request, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(pull_requests)?]))
    }

    #[tool(description = "Get details for a specific pull request.")]
    async fn git_pr_details_get(&self, request: GetPullRequestDetailsRequest) -> McpResult {
        let pull_request =
            git::bridge::get_pull_request(self.client.clone(), request, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(pull_request)?]))
    }

    #[tool(description = "Get details for a specific Backlog issue.")]
    async fn issue_details_get(&self, request: GetIssueDetailsRequest) -> McpResult {
        let issue =
            issue::bridge::get_issue_details(self.client.clone(), request, &self.access_control)
                .await?;
        let issue_response = IssueResponse::from(issue);
        Ok(CallToolResult::success(vec![Content::json(
            issue_response,
        )?]))
    }

    #[tool(description = "Get details for a specific Backlog document.
     This API returns the document details including its title, `plain` as Markdown and `json` as ProseMirror json, and other metadata.")]
    async fn document_details_get(&self, request: GetDocumentDetailsRequest) -> McpResult {
        let document = document::bridge::get_document_details(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;

        Ok(CallToolResult::success(vec![Content::json(document)?]))
    }

    #[tool(
        description = "Download a document attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn document_attachment_download(
        &self,
        request: DownloadDocumentAttachmentRequest,
    ) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = document::bridge::download_document_attachment_bridge(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get the document tree for a specified project.")]
    async fn document_tree_get(&self, request: GetDocumentTreeRequest) -> McpResult {
        let document_tree = document::bridge::get_document_tree_tool(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(
            document_tree.active_tree,
        )?]))
    }

    #[tool(description = "Get a list of versions (milestones) for a specified project.")]
    async fn issue_milestone_list_get(&self, request: GetVersionMilestoneListRequest) -> McpResult {
        let milestones = issue::bridge::get_version_milestone_list(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(milestones)?]))
    }

    #[tool(description = "Get a list of issues for a specified milestone name within a project.")]
    async fn issue_list_by_milestone_get(
        &self,
        request: GetIssuesByMilestoneNameRequest,
    ) -> McpResult {
        let issues = issue::bridge::get_issues_by_milestone_name(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        let issue_responses: Vec<IssueResponse> =
            issues.into_iter().map(IssueResponse::from).collect();
        Ok(CallToolResult::success(vec![Content::json(
            issue_responses,
        )?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Update the summary and/or description of a Backlog issue.")]
    async fn issue_update(&self, request: UpdateIssueRequest) -> McpResult {
        let updated_issue =
            issue::bridge::update_issue_impl(self.client.clone(), request, &self.access_control)
                .await?;
        let issue_response = IssueResponse::from(updated_issue);
        Ok(CallToolResult::success(vec![Content::json(
            issue_response,
        )?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Add a comment to a Backlog issue.")]
    async fn issue_comment_add(&self, request: AddCommentRequest) -> McpResult {
        let comment =
            issue::bridge::add_comment_impl(self.client.clone(), request, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Update an existing comment on a Backlog issue.")]
    async fn issue_comment_update(&self, request: UpdateCommentRequest) -> McpResult {
        let comment =
            issue::bridge::update_comment_impl(self.client.clone(), request, &self.access_control)
                .await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Create a new issue in a Backlog project.")]
    async fn issue_add(&self, request: AddIssueRequest) -> McpResult {
        let issue =
            issue::bridge::add_issue_impl(self.client.clone(), request, &self.access_control)
                .await?;
        let issue_response = IssueResponse::from(issue);
        Ok(CallToolResult::success(vec![Content::json(
            issue_response,
        )?]))
    }

    #[tool(
        name = "issue_comment_list_get",
        description = "Gets comments for a specific issue. Takes 'issue_id_or_key' (string, required) and optional 'min_id', 'max_id', 'count', 'order' parameters."
    )]
    async fn issue_comment_list_get(&self, request: GetIssueCommentsRequest) -> McpResult {
        let comments = issue::bridge::get_issue_comments_impl(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(comments)?]))
    }

    #[tool(
        name = "issue_attachment_list_get",
        description = "Get a list of attachments for a specified issue."
    )]
    async fn issue_attachment_list_get(&self, request: GetAttachmentListRequest) -> McpResult {
        let attachments = issue::bridge::get_attachment_list_impl(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        name = "issue_shared_file_list_get",
        description = "Get a list of shared files linked to a specified issue."
    )]
    async fn issue_shared_file_list_get(&self, request: GetIssueSharedFilesRequest) -> McpResult {
        let shared_files = issue::bridge::get_issue_shared_files_impl(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(shared_files)?]))
    }

    #[tool(description = "Get a list of users in the space.")]
    async fn user_list_get(&self, request: GetUserListRequest) -> McpResult {
        let users = user::bridge::get_user_list_bridge(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(users)?]))
    }

    #[tool(
        description = "Download an issue attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn issue_attachment_download(&self, request: DownloadAttachmentRequest) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = issue::bridge::download_issue_attachment_file(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get a list of statuses for a specified project.")]
    async fn project_status_list_get(&self, request: GetProjectStatusListRequest) -> McpResult {
        let statuses = project::bridge::get_project_status_list_tool(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(statuses)?]))
    }

    #[tool(description = "Get a list of issue types for a specified project.")]
    async fn project_issue_type_list_get(&self, request: GetProjectIssueTypesRequest) -> McpResult {
        let issue_types = project::bridge::get_project_issue_types_tool(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(issue_types)?]))
    }

    #[tool(description = "Get a list of priorities available in Backlog.")]
    async fn issue_priority_list_get(&self, request: GetPrioritiesRequest) -> McpResult {
        let priorities = project::bridge::get_priorities_tool(self.client.clone(), request).await?;
        Ok(CallToolResult::success(vec![Content::json(priorities)?]))
    }

    #[tool(description = "Get a list of custom fields for a specified project with examples.")]
    async fn project_custom_field_list_get(&self, request: GetCustomFieldListRequest) -> McpResult {
        let custom_fields = project::bridge::get_custom_field_list_tool(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(custom_fields)?]))
    }

    #[tool(description = "Get a list of shared files for a specified project directory.")]
    async fn file_shared_list_get(&self, request: GetSharedFilesListRequest) -> McpResult {
        let files = file::bridge::get_shared_files_list_tool(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(files)?]))
    }

    #[tool(
        description = "Download a shared file. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn file_shared_download(&self, request: DownloadSharedFileRequest) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = file::bridge::download_shared_file_bridge(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get a list of attachments for a specific pull request.")]
    async fn git_pr_attachment_list_get(
        &self,
        request: GetPullRequestAttachmentListRequest,
    ) -> McpResult {
        let attachments = git::bridge::get_pull_request_attachment_list_tool(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        description = "Download a pull request attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn git_pr_attachment_download(
        &self,
        request: DownloadPullRequestAttachmentRequest,
    ) -> McpResult {
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file = git::bridge::download_pr_attachment_bridge(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[tool(description = "Get a list of comments for a specific pull request.")]
    async fn git_pr_comment_list_get(
        &self,
        request: GetPullRequestCommentListRequest,
    ) -> McpResult {
        let comments = git::bridge::get_pull_request_comment_list_tool(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(comments)?]))
    }

    #[tool(description = "Get detailed information about a specific wiki page.")]
    async fn wiki_details_get(&self, request: GetWikiDetailRequest) -> McpResult {
        let client = self.client.lock().await;
        let detail = wiki::bridge::get_wiki_detail(&client, request, &self.access_control).await?;

        Ok(CallToolResult::success(vec![Content::json(detail)?]))
    }

    #[tool(description = "Get a list of wiki pages. Can be filtered by project and keyword.")]
    async fn wiki_list_get(&self, request: GetWikiListRequest) -> McpResult {
        let client = self.client.lock().await;
        let wikis = wiki::bridge::get_wiki_list(&client, request, &self.access_control).await?;

        Ok(CallToolResult::success(vec![Content::json(wikis)?]))
    }

    #[tool(description = "Get a list of attachments for a specified wiki page.")]
    async fn wiki_attachment_list_get(&self, request: GetWikiAttachmentListRequest) -> McpResult {
        let client = self.client.lock().await;
        let attachments =
            wiki::bridge::get_wiki_attachment_list(&client, request, &self.access_control).await?;
        Ok(CallToolResult::success(vec![Content::json(attachments)?]))
    }

    #[tool(
        description = "Download an attachment from a wiki page. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format."
    )]
    async fn wiki_attachment_download(&self, request: DownloadWikiAttachmentRequest) -> McpResult {
        let client = self.client.lock().await;
        let explicit_format = request
            .format
            .as_deref()
            .map(str::parse::<FileFormat>)
            .transpose()?;

        let file =
            wiki::bridge::download_wiki_attachment(&client, request, &self.access_control).await?;

        let response_data = SerializableFile::new(file, explicit_format)?;
        Ok(CallToolResult::success(vec![response_data.try_into()?]))
    }

    #[cfg(feature = "wiki_writable")]
    #[tool(
        description = "Update a wiki page. You can update the page name, content, and/or email notification settings."
    )]
    async fn wiki_update(&self, request: UpdateWikiRequest) -> McpResult {
        let client = self.client.lock().await;
        let wiki_detail = wiki::bridge::update_wiki(&client, request, &self.access_control).await?;
        Ok(CallToolResult::success(vec![Content::json(wiki_detail)?]))
    }

    #[cfg(feature = "git_writable")]
    #[tool(
        description = "Add a comment to a specific pull request. Optionally notify specified users."
    )]
    async fn git_pr_comment_add(&self, request: AddPullRequestCommentRequest) -> McpResult {
        let comment = git::bridge::add_pull_request_comment_bridge(
            self.client.clone(),
            request,
            &self.access_control,
        )
        .await?;
        Ok(CallToolResult::success(vec![Content::json(comment)?]))
    }
}

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
