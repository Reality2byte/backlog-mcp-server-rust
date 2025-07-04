use backlog_team::api::TeamApi;
use client::Client;
use wiremock::MockServer;

pub async fn setup_team_api(mock_server: &MockServer) -> TeamApi {
    let base_url = mock_server.uri();
    let client = Client::new(&base_url)
        .expect("Failed to create client")
        .with_api_key("test-api-key");
    TeamApi::new(client)
}
