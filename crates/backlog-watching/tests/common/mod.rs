use backlog_watching::WatchingApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

pub async fn setup_watching_api(mock_server: &MockServer) -> WatchingApi {
    let client = setup_client(mock_server)
        .await
        .with_api_key("dummy_api_key");
    WatchingApi::new(client)
}
