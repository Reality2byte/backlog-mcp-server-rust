use backlog_file::api::FileApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
pub async fn setup_file_api(mock_server: &MockServer) -> FileApi {
    let client = setup_client(mock_server).await;
    FileApi::new(client)
}

/// Common imports for tests
pub use backlog_core::identifier::{Identifier, ProjectId, SharedFileId, UserId};
pub use backlog_core::{Language, ProjectIdOrKey, Role, User};
pub use chrono::{TimeZone, Utc};
pub use wiremock::matchers::{method, path, query_param};
pub use wiremock::{Mock, ResponseTemplate};
