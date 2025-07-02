use backlog_project::api::ProjectApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function to create ProjectApi with mocked client
pub async fn setup_project_api(mock_server: &MockServer) -> ProjectApi {
    let client = setup_client(mock_server).await;
    ProjectApi::new(client)
}
