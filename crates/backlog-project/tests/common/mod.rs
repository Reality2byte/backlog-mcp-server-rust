use backlog_project::api::ProjectApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function to create ProjectApi with mocked client
pub async fn setup_project_api(mock_server: &MockServer) -> ProjectApi {
    let client = setup_client(mock_server).await;
    ProjectApi::new(client)
}

/// Common imports for tests
pub use backlog_core::identifier::{
    CategoryId, IssueTypeId, MilestoneId, PriorityId, ProjectId, ResolutionId, StatusId,
};
pub use backlog_domain_models::Milestone;
pub use chrono::TimeZone;
pub use wiremock::matchers::{method, path};
pub use wiremock::{Mock, ResponseTemplate};
