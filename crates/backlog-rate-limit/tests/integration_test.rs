use backlog_api_client::client::BacklogApiClient;

/// Integration test that runs when BACKLOG_BASE_URL and BACKLOG_API_KEY are set
#[tokio::test]
async fn test_get_rate_limit_integration() {
    // Only run if environment variables are set
    let base_url = match std::env::var("BACKLOG_BASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Skipping integration test: BACKLOG_BASE_URL not set");
            return;
        }
    };

    let api_key = match std::env::var("BACKLOG_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping integration test: BACKLOG_API_KEY not set");
            return;
        }
    };

    let client = BacklogApiClient::new(&base_url)
        .expect("Failed to create client")
        .with_api_key(api_key);

    let result = client.rate_limit().get_rate_limit().await;

    match result {
        Ok(response) => {
            println!("Successfully retrieved rate limit information");
            println!(
                "Read limit: {}/{}",
                response.rate_limit.read.remaining, response.rate_limit.read.limit
            );
            println!(
                "Update limit: {}/{}",
                response.rate_limit.update.remaining, response.rate_limit.update.limit
            );
            println!(
                "Search limit: {}/{}",
                response.rate_limit.search.remaining, response.rate_limit.search.limit
            );
            println!(
                "Icon limit: {}/{}",
                response.rate_limit.icon.remaining, response.rate_limit.icon.limit
            );

            // Verify all fields are present and make sense
            assert!(response.rate_limit.read.limit > 0);
            assert!(response.rate_limit.read.remaining >= 0);
            assert!(response.rate_limit.read.remaining <= response.rate_limit.read.limit);
            assert!(response.rate_limit.read.reset > 0);

            assert!(response.rate_limit.update.limit > 0);
            assert!(response.rate_limit.update.remaining >= 0);
            assert!(response.rate_limit.update.remaining <= response.rate_limit.update.limit);
            assert!(response.rate_limit.update.reset > 0);

            assert!(response.rate_limit.search.limit > 0);
            assert!(response.rate_limit.search.remaining >= 0);
            assert!(response.rate_limit.search.remaining <= response.rate_limit.search.limit);
            assert!(response.rate_limit.search.reset > 0);

            assert!(response.rate_limit.icon.limit > 0);
            assert!(response.rate_limit.icon.remaining >= 0);
            assert!(response.rate_limit.icon.remaining <= response.rate_limit.icon.limit);
            assert!(response.rate_limit.icon.reset > 0);
        }
        Err(e) => {
            panic!("Failed to get rate limit: {e}");
        }
    }
}
