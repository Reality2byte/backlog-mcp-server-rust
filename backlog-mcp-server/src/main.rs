use rmcp::{ServiceExt, transport::stdio};

use mcp_backlog_server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Backlog MCP Server starting...");

    let server = match Server::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize server: {e}");
            eprintln!(
                "Please ensure BACKLOG_BASE_URL and BACKLOG_API_KEY environment variables are set"
            );
            return Err(e);
        }
    };

    let service = server.serve(stdio()).await.inspect_err(|e| {
        eprintln!("Error starting server: {e}");
    })?;
    service.waiting().await?;

    eprintln!("Backlog MCP Server finished.");
    Ok(())
}
