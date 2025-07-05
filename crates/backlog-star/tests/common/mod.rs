use backlog_star::StarApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
pub async fn setup_star_api(mock_server: &MockServer) -> StarApi {
    let client = setup_client(mock_server).await;
    StarApi::new(client)
}

/// Common imports for tests
pub use wiremock::{Mock, ResponseTemplate};
