use crate::{
    git::{
        self,
        request::{
            GetPullRequestDetailsRequest, GetRepositoryDetailsRequest, GetRepositoryListRequest,
            ListPullRequestsRequest,
        },
    },
};
use rmcp::{Error as McpError, model::*, tool};
use crate::server::Server;

#[tool(tool_box)]
impl Server {

    #[tool(description = "Get a list of Git repositories for a specified project.")]
    async fn get_repository_listx(
        &self,
        #[tool(aggr)] request: GetRepositoryListRequest, // Changed from git_tools
    ) -> Result<CallToolResult, McpError> {
        let repositories =
            git::bridge::get_repository_list_impl(self.client.clone(), request.project_id_or_key)
                .await?;
        Ok(CallToolResult::success(vec![
            Content::json(repositories).unwrap(),
        ]))
    }
}
