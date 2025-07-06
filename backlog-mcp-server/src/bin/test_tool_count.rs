use mcp_backlog_server::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set environment variables
    unsafe {
        std::env::set_var("BACKLOG_BASE_URL", "https://test.backlog.jp");
        std::env::set_var("BACKLOG_API_KEY", "test_key");
    }

    // Create server
    let server = Server::new()?;

    // Count tools using the tool_router field
    let tool_count = server.tool_router.map.len();

    println!("Total tools registered: {tool_count}");

    // List all tool names
    println!("\nRegistered tools:");
    for name in server.tool_router.map.keys() {
        println!("  - {name}");
    }

    Ok(())
}
