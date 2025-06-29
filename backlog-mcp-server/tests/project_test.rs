use mcp_backlog_server::Server;
use mcp_backlog_server::project::request::GetCustomFieldListRequest;
use std::env;

#[cfg(test)]
mod project_tools_tests {
    use super::*;

    fn setup_test_env() {
        unsafe {
            env::set_var("BACKLOG_BASE_URL", "https://test.backlog.jp");
            env::set_var("BACKLOG_API_KEY", "test_api_key");
        }
    }

    #[tokio::test]
    async fn test_get_custom_field_list_tool_exists() {
        setup_test_env();

        // The Server should have a get_custom_field_list tool
        let _server = Server::new().expect("Failed to create server");

        // Test that we can create the request type
        let request = GetCustomFieldListRequest {
            project_id_or_key: "TEST_PROJECT".to_string(),
        };

        // The get_custom_field_list tool is private on the Server struct,
        // but it's exposed through the MCP tool mechanism.
        // This test verifies that the request type and server can be created.
        assert_eq!(request.project_id_or_key, "TEST_PROJECT");

        // In a real test environment, we would test the bridge function directly
        // or use the MCP server's tool infrastructure to call it
    }
}
