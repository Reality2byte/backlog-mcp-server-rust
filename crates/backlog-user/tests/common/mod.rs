use backlog_user::api::UserApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function to create UserApi with mocked client
pub async fn setup_user_api(mock_server: &MockServer) -> UserApi {
    let client = setup_client(mock_server).await;
    UserApi::new(client)
}
