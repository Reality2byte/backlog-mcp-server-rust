use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::ProjectId;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url =
        env::var("BACKLOG_BASE_URL").expect("BACKLOG_BASE_URL environment variable is required");

    let api_key =
        env::var("BACKLOG_API_KEY").expect("BACKLOG_API_KEY environment variable is required");

    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);

    /*let space = client.space().get_space().await?;
    println!("Space information: {:?}", space);

    let user = client.user().get_own_user().await?;
    println!("User information: {:?}", user);

    let project = client
        .project()
        .get_project(ProjectIdOrKey::from_str("MFP").unwrap())
        .await?;
    println!("Project information: {:?}", project);

    let projects = client
        .project()
        .get_project_list(GetProjectParams {
            ..Default::default()
        })
        .await?;
    projects.iter().for_each(|project| {
        println!("Project information: {:?}", project);
    });

    let issue = client
        .issue()
        .get_issue(IssueKey::from_str("MFP-1").unwrap())
        .await?;
    println!("Issue information: {:?}", issue);*/

    let filter = backlog_issue::requests::CountIssueParamsBuilder::default()
        .project_id(vec![14165.into()])
        .build()
        .unwrap();
    println!("Count of issues: {:?}", filter);
    let count = client.issue().count_issue(filter).await?;

    println!("Count of issues: {:?}", count);

    let params = backlog_document::requests::ListDocumentsParamsBuilder::default()
        .project_id(ProjectId::from(601486))
        .offset(0)
        .count(10)
        .build()
        .unwrap();
    let documents = client.document().list_documents(params).await?;
    println!("documents: {:?}", documents);

    /*let issue = client
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
        .await?;
    println!("Created issue: {:?}", issue);

    let issue = client.issue().delete_issue(issue.issue_key.clone()).await?;
    println!("Deleted issue: {:?}", issue);
    */

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
