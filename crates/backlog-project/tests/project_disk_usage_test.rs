use backlog_api_core::Error as ApiError;
use backlog_core::{
    ProjectKey,
    identifier::{Identifier, ProjectId},
};
use backlog_project::api::{GetProjectDiskUsageParams, ProjectApi};
use backlog_space::api::ProjectDiskUsage;
use client::test_utils::setup_client;
use std::str::FromStr;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub type GetProjectDiskUsageResponse = ProjectDiskUsage;

#[tokio::test]
async fn test_get_project_disk_usage_success() {
    let mock_server = MockServer::start().await;
    let client = setup_client(&mock_server).await;
    let project_api = ProjectApi::new(client);

    let expected_response = serde_json::json!({
        "projectId": 123,
        "issue": 1288490188,
        "wiki": 524288000,
        "document": 314572800,
        "file": 419430400,
        "subversion": 10485760,
        "git": 209715200,
        "gitLFS": 31457280
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TEST_PROJECT/diskUsage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetProjectDiskUsageParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap());
    let result = project_api.get_disk_usage(params).await;
    assert!(result.is_ok());

    let disk_usage = result.unwrap();
    assert_eq!(disk_usage.project_id.value(), 123);
    assert_eq!(disk_usage.issue, 1288490188);
    assert_eq!(disk_usage.wiki, 524288000);
    assert_eq!(disk_usage.document, 314572800);
    assert_eq!(disk_usage.file, 419430400);
    assert_eq!(disk_usage.subversion, 10485760);
    assert_eq!(disk_usage.git, 209715200);
    assert_eq!(disk_usage.git_lfs, 31457280);
}

#[tokio::test]
async fn test_get_project_disk_usage_by_id() {
    let mock_server = MockServer::start().await;
    let client = setup_client(&mock_server).await;
    let project_api = ProjectApi::new(client);

    let expected_response = serde_json::json!({
        "projectId": 12345,
        "issue": 0,
        "wiki": 0,
        "document": 0,
        "file": 0,
        "subversion": 0,
        "git": 0,
        "gitLFS": 0
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/12345/diskUsage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetProjectDiskUsageParams::new(ProjectId::new(12345));
    let result = project_api.get_disk_usage(params).await;
    assert!(result.is_ok());

    let disk_usage = result.unwrap();
    assert_eq!(disk_usage.project_id.value(), 12345);
    assert_eq!(disk_usage.issue, 0);
    assert_eq!(disk_usage.wiki, 0);
}

#[tokio::test]
async fn test_get_project_disk_usage_unauthorized() {
    let mock_server = MockServer::start().await;
    let client = setup_client(&mock_server).await;
    let project_api = ProjectApi::new(client);

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "You do not have permission to view disk usage for this project",
                "code": 6
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/TEST_PROJECT/diskUsage"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_response))
        .mount(&mock_server)
        .await;

    let params = GetProjectDiskUsageParams::new(ProjectKey::from_str("TEST_PROJECT").unwrap());
    let result = project_api.get_disk_usage(params).await;
    assert!(result.is_err());

    match result {
        Err(ApiError::HttpStatus { status, .. }) => {
            assert_eq!(status, 401);
        }
        _ => panic!("Expected HttpStatus error with 401"),
    }
}

#[tokio::test]
async fn test_get_project_disk_usage_not_found() {
    let mock_server = MockServer::start().await;
    let client = setup_client(&mock_server).await;
    let project_api = ProjectApi::new(client);

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "No project found",
                "code": 7
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/NONEXISTENT/diskUsage"))
        .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
        .mount(&mock_server)
        .await;

    let params = GetProjectDiskUsageParams::new(ProjectKey::from_str("NONEXISTENT").unwrap());
    let result = project_api.get_disk_usage(params).await;
    assert!(result.is_err());

    match result {
        Err(ApiError::HttpStatus { status, .. }) => {
            assert_eq!(status, 404);
        }
        _ => panic!("Expected HttpStatus error with 404"),
    }
}
