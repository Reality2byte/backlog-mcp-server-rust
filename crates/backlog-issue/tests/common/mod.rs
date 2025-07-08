use backlog_issue::api::IssueApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
pub async fn setup_issue_api(mock_server: &MockServer) -> IssueApi {
    let client = setup_client(mock_server).await;
    IssueApi::new(client)
}

/// Common imports for tests
#[allow(unused_imports)]
pub use backlog_core::identifier::{
    AttachmentId, CommentId, IssueId, ProjectId, SharedFileId, UserId,
};
#[allow(unused_imports)]
pub use backlog_core::{IssueIdOrKey, Language, Role, User};
#[allow(unused_imports)]
pub use backlog_issue::models::{Attachment, Comment, FileContent, Issue, SharedFile};
#[allow(unused_imports)]
pub use chrono::{TimeZone, Utc};
#[allow(unused_imports)]
pub use serde_json::json;
#[allow(unused_imports)]
pub use wiremock::matchers::{method, path, query_param};
#[allow(unused_imports)]
pub use wiremock::{Mock, ResponseTemplate};
