use backlog_api_client::client::BacklogApiClient;
use backlog_core::IssueKey;
use rmcp::{Error as McpError, model::*, schemars, tool};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

//use crate::tools;

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

    #[tool(description = "Get details for a specific Backlog issue")]
    async fn get_issue_details(
        &self,
        #[tool(aggr)] GetIssueDetailsRequest { issue_key }: GetIssueDetailsRequest,
    ) -> Result<CallToolResult, McpError> {
        let client = self.client.lock().await;

        let issue_key = IssueKey::from_str(issue_key.trim()).map_err(|e| {
            McpError::invalid_request(
                format!("Invalid issue key: {:?}", issue_key),
                Some(serde_json::Value::String(e.to_string())),
            )
        })?;
        match client.issue().get_issue(issue_key.clone()).await {
            Ok(issue) => Ok(CallToolResult::success(vec![Content::json(issue)?])),
            Err(e) => Err(McpError::resource_not_found(
                format!("No such issue found: {:?}", issue_key),
                Some(serde_json::Value::String(e.to_string())),
            )),
        }
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
