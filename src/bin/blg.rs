use backlog_api_client::api;
use backlog_api_client::client::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url =
        env::var("BACKLOG_BASE_URL").expect("BACKLOG_BASE_URL environment variable is required");

    let api_key =
        env::var("BACKLOG_API_KEY").expect("BACKLOG_API_KEY environment variable is required");

    let client = Client::new(&base_url)?.with_api_key(api_key);

    match api::get_space(&client).await {
        Ok(space) => {
            println!("Space information:");
            println!("Name: {}", space.name);
            println!("Space key: {}", space.space_key);
            println!("Created at: {}", space.created);
        }
        Err(e) => {
            eprintln!("Error getting space information: {}", e);
        }
    }

    Ok(())
}
