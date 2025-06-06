// This module provides test utilities for other crates in the workspace.

use crate::client::Client; // Use Client from the same crate
use wiremock::MockServer; // Removed cfg attribute

/// Creates a new `Client` instance for testing, configured to use the provided `MockServer`.
pub async fn setup_client(mock_server: &MockServer) -> Client {
    // Removed cfg attribute
    Client::new(&mock_server.uri()).expect("Failed to create client for mock server")
}
