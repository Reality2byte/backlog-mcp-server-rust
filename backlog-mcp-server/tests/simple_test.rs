use mcp_backlog_server::Server;

#[test]
fn test_server_creation() {
    // Just test that the server can be created
    unsafe {
        std::env::set_var("BACKLOG_BASE_URL", "https://test.backlog.jp");
        std::env::set_var("BACKLOG_API_KEY", "test_key");
    }

    let server = Server::new();
    assert!(server.is_ok(), "Server creation should succeed");
}

#[test]
fn test_tool_router_exists() {
    unsafe {
        std::env::set_var("BACKLOG_BASE_URL", "https://test.backlog.jp");
        std::env::set_var("BACKLOG_API_KEY", "test_key");
    }

    let server = Server::new().expect("Failed to create server");

    // Just verify the tool_router field exists and is accessible
    let _ = &server.tool_router;

    // If we get here without panic, the test passes
}
