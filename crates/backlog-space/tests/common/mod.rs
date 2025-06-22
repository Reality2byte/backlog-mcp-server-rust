use backlog_space::api::SpaceApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function to create SpaceApi with mocked client
pub async fn setup_space_api(mock_server: &MockServer) -> SpaceApi {
    let client = setup_client(mock_server).await;
    SpaceApi::new(client)
}

/// Common imports for tests
pub use wiremock::matchers::{method, path};
pub use wiremock::{Mock, ResponseTemplate};
