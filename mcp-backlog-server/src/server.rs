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

// Request struct for get_repository_list is now in git_tools.rs
// We need to import it if it's not already covered by `use crate::git_tools;`
// For clarity, let's assume git_tools::GetRepositoryListRequest will be used directly.
#[tool(tool_box)]
impl Server {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = env::var("BACKLOG_BASE_URL")
            .expect("BACKLOG_BASE_URL environment variable is required");
        let api_key =
            env::var("BACKLOG_API_KEY").expect("BACKLOG_API_KEY environment variable is required");

        let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    #[tool(description = "Get a list of Git repositories for a specified project.")]
    async fn get_repository_list(
        &self,
        #[tool(aggr)] request: GetRepositoryListRequest,
    ) -> Result<CallToolResult, McpError> {
        let repositories =
            git::bridge::get_repository_list_impl(self.client.clone(), request.project_id_or_key)
                .await?;
        Ok(CallToolResult::success(vec![
            Content::json(repositories).unwrap(),
        ]))
    }

    #[tool(description = "Get details for a specific Git repository.")]
    async fn get_repository_details(
        &self,
        #[tool(aggr)] request: GetRepositoryDetailsRequest,
    ) -> Result<CallToolResult, McpError> {
        let repository = git::bridge::get_repository_details_impl(
            self.client.clone(),
            request.project_id_or_key,
            request.repo_id_or_name,
        )
        .await?;
        Ok(CallToolResult::success(vec![
            Content::json(repository).unwrap(),
        ]))
    }

    #[tool(description = "Get a list of pull requests for a specified repository.")]
    async fn list_pull_requests(
        &self,
        #[tool(aggr)] request: ListPullRequestsRequest,
    ) -> Result<CallToolResult, McpError> {
        let pull_requests = git::bridge::list_pull_requests_impl(
            self.client.clone(),
            request.project_id_or_key,
            request.repo_id_or_name,
        )
        .await?;
        Ok(CallToolResult::success(vec![
            Content::json(pull_requests).unwrap(),
        ]))
    }

    #[tool(description = "Get details for a specific pull request.")]
    async fn get_pull_request_details(
        &self,
        #[tool(aggr)] request: GetPullRequestDetailsRequest,
    ) -> Result<CallToolResult, McpError> {
        let pull_request = git::bridge::get_pull_request_details_impl(
            self.client.clone(),
            request.project_id_or_key,
            request.repo_id_or_name,
            request.pr_number,
        )
        .await?;
        Ok(CallToolResult::success(vec![
            Content::json(pull_request).unwrap(),
        ]))
    }

    #[tool(description = "Get details for a specific Backlog issue.")]
    async fn get_issue_details(
        &self,
        #[tool(aggr)] GetIssueDetailsRequest { issue_key }: GetIssueDetailsRequest,
    ) -> Result<CallToolResult, McpError> {
        let issue = issue::bridge::get_issue_details(self.client.clone(), issue_key).await?;
        Ok(CallToolResult::success(vec![Content::json(issue).unwrap()]))
    }

    #[tool(description = "Get details for a specific Backlog document.
     This API returns the document details including its title, `plain` as Markdown and `json` as ProseMirror json, and other metadata.")]
    async fn get_document_details(
        &self,
        #[tool(aggr)] GetDocumentDetailsRequest { document_id }: GetDocumentDetailsRequest,
    ) -> Result<CallToolResult, McpError> {
        let document =
            document::bridge::get_document_details(self.client.clone(), document_id).await?;
        Ok(CallToolResult::success(vec![
            Content::json(document).unwrap(),
        ]))
    }

    #[tool(description = "Get a list of versions (milestones) for a specified project.")]
    async fn get_version_milestone_list(
        &self,
        #[tool(aggr)]
        GetVersionMilestoneListRequest {
            project_id_or_key,
        }: GetVersionMilestoneListRequest,
    ) -> Result<CallToolResult, McpError> {
        let milestones =
            issue::bridge::get_version_milestone_list_impl(self.client.clone(), project_id_or_key)
                .await?;
        Ok(CallToolResult::success(vec![
            Content::json(milestones).unwrap(),
        ]))
    }

    #[tool(description = "Get a list of issues for a specified milestone name within a project.")]
    async fn get_issues_by_milestone_name(
        &self,
        #[tool(aggr)] GetIssuesByMilestoneNameRequest {
            project_id_or_key,
            milestone_name,
        }: GetIssuesByMilestoneNameRequest,
    ) -> Result<CallToolResult, McpError> {
        let issues = issue::bridge::get_issues_by_milestone_name_impl(
            self.client.clone(),
            project_id_or_key,
            milestone_name,
        )
        .await?;
        Ok(CallToolResult::success(vec![
            Content::json(issues).unwrap(),
        ]))
    }

    #[cfg(feature = "issue_writable")]
    #[tool(description = "Update the summary and/or description of a Backlog issue.")]
    async fn update_issue(
        &self,
        #[tool(aggr)] req: UpdateIssueRequest,
    ) -> Result<CallToolResult, McpError> {
        let updated_issue = issue::bridge::update_issue_impl(
            self.client.clone(),
            req.issue_id_or_key,
            req.summary,
            req.description,
        )
        .await?;
        Ok(CallToolResult::success(vec![
            Content::json(updated_issue).unwrap(),
        ]))
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
