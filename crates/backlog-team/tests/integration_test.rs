/// Integration test for team API
///
/// This test requires:
/// - BACKLOG_BASE_URL environment variable set to your Backlog space URL
/// - BACKLOG_API_KEY environment variable set to your API key
/// - Administrator permission
/// - A valid team ID in your Backlog space
///
/// Run with:
/// ```
/// BACKLOG_BASE_URL=https://yourspace.backlog.jp BACKLOG_API_KEY=your_key cargo test --package backlog-team --test integration_test -- --nocapture
/// ```
#[cfg(test)]
mod tests {
    use backlog_api_client::client::BacklogApiClient;
    use backlog_core::{id::TeamId, identifier::Identifier};
    use backlog_team::api::GetTeamParams;

    fn skip_if_no_env() -> Option<(String, String)> {
        let base_url = std::env::var("BACKLOG_BASE_URL").ok()?;
        let api_key = std::env::var("BACKLOG_API_KEY").ok()?;
        Some((base_url, api_key))
    }

    #[tokio::test]
    #[ignore] // Run manually with proper environment variables
    async fn test_get_team_integration() {
        let (base_url, api_key) = match skip_if_no_env() {
            Some(v) => v,
            None => {
                eprintln!("Skipping integration test: BACKLOG_BASE_URL or BACKLOG_API_KEY not set");
                return;
            }
        };

        let client = BacklogApiClient::new(&base_url)
            .expect("Failed to create client")
            .with_api_key(api_key);

        let api = client.team();

        // You need to replace this with an actual team ID from your Backlog space
        let team_id = match std::env::var("BACKLOG_TEST_TEAM_ID") {
            Ok(id) => id.parse().expect("Invalid team ID"),
            Err(_) => {
                eprintln!("Skipping integration test: BACKLOG_TEST_TEAM_ID not set");
                eprintln!(
                    "Please set BACKLOG_TEST_TEAM_ID to a valid team ID in your Backlog space"
                );
                return;
            }
        };

        let params = GetTeamParams {
            team_id: TeamId::new(team_id),
        };

        match api.get_team(params).await {
            Ok(team) => {
                println!("Successfully retrieved team:");
                println!("  ID: {}", team.id.value());
                println!("  Name: {}", team.name);
                println!("  Members: {}", team.members.len());

                assert!(!team.name.is_empty());
                assert!(team.id.value() > 0);
            }
            Err(e) => {
                eprintln!("Failed to get team: {e}");
                panic!("Integration test failed");
            }
        }
    }
}
