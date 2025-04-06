use backlog_api_client::client::BacklogApiClient;
use backlog_core::{Identifier, IssueKey, ProjectIdOrKey};
use backlog_project::requests::GetProjectParams;

use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url =
        env::var("BACKLOG_BASE_URL").expect("BACKLOG_BASE_URL environment variable is required");

    let api_key =
        env::var("BACKLOG_API_KEY").expect("BACKLOG_API_KEY environment variable is required");

    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);

    match client.space().get_space().await {
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

    println!("------------------------");
    match client.user().get_own_user().await {
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

    println!("------------------------");
    match client
        .project()
        .get_project(ProjectIdOrKey::from_str("MFP").unwrap())
        .await
    {
        Ok(project) => {
            println!("Project information:");
            println!("Name: {}", project.name);
            println!("Key: {}", project.project_key);
        }
        Err(e) => {
            eprintln!("Error getting project information: {}", e);
        }
    }

    println!("------------------------");
    match client
        .project()
        .get_project_list(GetProjectParams {
            ..Default::default()
        })
        .await
    {
        Ok(projects) => {
            for project in projects {
                println!("Project information:");
                println!("Name: {}", project.name);
                println!("Key: {}", project.project_key);
            }
        }
        Err(e) => {
            eprintln!("Error getting user information: {}", e);
        }
    }

    println!("------------------------");
    match client
        .issue()
        .get_issue(IssueKey::from_str("MFP-1").unwrap())
        .await
    {
        Ok(issue) => {
            println!("Issue information:");
            println!("Name: {}", issue.issue_key);
            println!("Summary: {}", issue.summary);
            println!("Assignee: {:?}", issue.assignee);
        }
        Err(e) => {
            eprintln!("Error getting issue information: {}", e);
        }
    }

    println!("------------------------");
    match client
        .issue()
        .add_issue(
            backlog_issue::requests::AddIssueParamsBuilder::default()
                .project_id(14165)
                .summary("Test issue")
                .issue_type_id(56740)
                .priority_id(3)
                .description("Test issue description")
                .build()
                .unwrap(),
        )
        .await
    {
        Ok(issue) => {
            println!("Issue information:");
            println!("Name: {}", issue.issue_key);
            println!("Summary: {}", issue.summary);
            println!("Desc: {:?}", issue.description);
        }
        Err(e) => {
            eprintln!("Error getting issue information: {}", e);
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
