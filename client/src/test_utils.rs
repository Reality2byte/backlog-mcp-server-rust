// This module provides test utilities for other crates in the workspace.

use crate::client::Client;
use wiremock::MockServer;

/// Creates a new `Client` instance for testing, configured to use the provided `MockServer`.
pub async fn setup_client(mock_server: &MockServer) -> Client {
    Client::new(&mock_server.uri()).expect("Failed to create client for mock server")
}
