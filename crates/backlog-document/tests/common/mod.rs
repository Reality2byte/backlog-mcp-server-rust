use backlog_document::api::DocumentApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
pub async fn setup_document_api(mock_server: &MockServer) -> DocumentApi {
    let client = setup_client(mock_server).await;
    DocumentApi::new(client)
}

/// Common imports for tests
pub use wiremock::matchers::{method, path};
pub use wiremock::{Mock, ResponseTemplate};
