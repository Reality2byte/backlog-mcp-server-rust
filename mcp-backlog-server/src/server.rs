use crate::issue::request::UpdateIssueRequest;
use crate::{
    document::{self, request::GetDocumentDetailsRequest},
    git::{
        self,
        request::{
            GetPullRequestDetailsRequest, GetRepositoryDetailsRequest, GetRepositoryListRequest,
            ListPullRequestsRequest,
        },
    },
    issue::{
        self,
        request::{
            GetIssueDetailsRequest, GetIssuesByMilestoneNameRequest, GetVersionMilestoneListRequest,
        },
    },
};
use backlog_api_client::client::BacklogApiClient;
use rmcp::{Error as McpError, model::*, tool};
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
