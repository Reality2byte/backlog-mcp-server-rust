use backlog_activity::ActivityApi;
use client::Client;
use wiremock::MockServer;

pub async fn setup_activity_api(mock_server: &MockServer) -> ActivityApi {
    let client = Client::new(&mock_server.uri()).expect("Failed to create client");
    ActivityApi::new(client)
}
