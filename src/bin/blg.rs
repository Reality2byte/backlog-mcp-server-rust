use backlog_api_client::api;
use backlog_api_client::client::Client;
use backlog_api_client::types::Identifier;
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
            println!("Key: {}", space.space_key);
            println!("Lang: {}", space.lang);
            println!("Space key: {}", space.space_key);
            println!("Created at: {}", space.created);
        }
        Err(e) => {
            eprintln!("Error getting space information: {}", e);
        }
    }

    match api::get_own_user(&client).await {
        Ok(user) => {
            println!("User information:");
            println!("Name: {}", user.name);
            println!("ID: {}", user.id.value());
            println!("Lang: {:?}", user.lang);
            println!("Mail: {}", user.mail_address);
        }
        Err(e) => {
            eprintln!("Error getting user information: {}", e);
        }
    }

    /*match api::get_recent_updates(&client).await {
        Ok(updates) => {
            println!("Recent updates:");
            for update in updates {
                println!("Update ID: {}", update.id);
                //println!("Content: {}", update.content);
                println!("Created at: {}", update.created);
            }
        }
        Err(e) => {
            eprintln!("Error getting recent updates: {}", e);

            match e {
                Error::Url(e) => {
                    println!("Url error: {}", e);
                }
                Error::Client(e) => {
                    println!("Client error: {}", e);
                }
                Error::Json(e) => {
                    println!("JSON error: {}", e);
                }
                Error::Http(e) => {
                    println!("HTTP error: {}", e);
                }
            }

            /*if let Some(source) = e.get_ref() {
                if let Some(location) = source.downcast_ref::<serde_json::Error>() {
                    if let Some((line, column)) = location.line_col() {
                        println!("Error occurred at line {}, column {}", line, column);
                    }
                }
            } */
        }
    }*/

    Ok(())
}
