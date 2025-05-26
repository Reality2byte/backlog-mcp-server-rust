use rmcp::{ServiceExt, transport::stdio};

mod server;
use server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Backlog MCP Server starting...");

    let service = Server::new()?.serve(stdio()).await.inspect_err(|e| {
        println!("Error starting server: {}", e);
    })?;
    service.waiting().await?;

    eprintln!("Backlog MCP Server finished.");
    Ok(())
}
