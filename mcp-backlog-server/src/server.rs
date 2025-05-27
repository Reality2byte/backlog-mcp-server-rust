use backlog_api_client::client::BacklogApiClient;
use rmcp::{Error as McpError, model::*, schemars, tool};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{document, issue};

#[derive(Clone)]
pub struct Server {
    client: Arc<Mutex<BacklogApiClient>>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetIssueDetailsRequest {
    #[schemars(description = "The issue key to retrieve details for. 
    This should be in the format 'PROJECT-123', where 'PROJECT' is the project key and '123' is the issue number. 
    Ensure there are no leading or trailing spaces.")]
    pub issue_key: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetDocumentDetailsRequest {
    #[schemars(description = "The document id to retrieve details for. 
    This should be in the format 32 digit hex string. Ensure there are no leading or trailing spaces.
    When you access https://example.backlog.com/document/PROJECT/0195faa11fcb7aaab4c4005a7ada4b6f,
    the document id is '0195faa11fcb7aaab4c4005a7ada4b6f'.")]
    pub document_id: String,
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

    #[tool(description = "Get details for a specific Backlog issue.")]
    async fn get_issue_details(
        &self,
        #[tool(aggr)] GetIssueDetailsRequest { issue_key }: GetIssueDetailsRequest,
    ) -> Result<CallToolResult, McpError> {
        let issue = issue::get_issue_details(self.client.clone(), issue_key).await?;
        Ok(CallToolResult::success(vec![Content::json(issue).unwrap()]))
    }

    #[tool(description = "Get details for a specific Backlog document.
     This API returns the document details including its title, `plain` as Markdown and `json` as ProseMirror json, and other metadata.")]
    async fn get_document_details(
        &self,
        #[tool(aggr)] GetDocumentDetailsRequest { document_id }: GetDocumentDetailsRequest,
    ) -> Result<CallToolResult, McpError> {
        let document = document::get_document_details(self.client.clone(), document_id).await?;
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
            issue::get_version_milestone_list_impl(self.client.clone(), project_id_or_key).await?;
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
        let issues = issue::get_issues_by_milestone_name_impl(
            self.client.clone(),
            project_id_or_key,
            milestone_name,
        )
        .await?;
        Ok(CallToolResult::success(vec![
            Content::json(issues).unwrap(),
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
