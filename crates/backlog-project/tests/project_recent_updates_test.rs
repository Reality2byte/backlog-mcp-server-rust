mod common;

use backlog_core::{
    ProjectKey,
    identifier::{ActivityId, ActivityTypeId, Identifier, ProjectId},
};
use backlog_project::api::GetProjectRecentUpdatesParams;
use common::*;
use std::str::FromStr;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_get_project_recent_updates_success() {
    let server = MockServer::start().await;
    let project_api = setup_project_api(&server).await;

    let mock_response = serde_json::json!([
        {
            "id": 143592,
            "project": {
                "id": 1,
                "projectKey": "EXAMPLE",
                "name": "Example Project",
                "chartEnabled": false,
                "subtaskingEnabled": false,
                "projectLeaderCanEditProjectLeader": false,
                "useWiki": true,
                "useFileSharing": true,
                "useWikiTreeView": false,
                "useOriginalImageSizeAtWiki": false,
                "textFormattingRule": "markdown",
                "archived": false,
                "displayOrder": 0,
                "useDevAttributes": false
            },
            "type": 1,
            "content": {
                "id": 1234,
                "key_id": 100,
                "summary": "Fix bug in login",
                "description": "Fixed authentication issue"
            },
            "notifications": [],
            "createdUser": {
                "id": 1001,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": null,
                "mailAddress": "admin@example.com",
                "lastLoginTime": "2024-01-01T00:00:00Z"
            },
            "created": "2024-01-15T10:30:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/EXAMPLE/activities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let params = GetProjectRecentUpdatesParams::new(ProjectKey::from_str("EXAMPLE").unwrap());
    let result = project_api.get_project_recent_updates(params).await;

    assert!(result.is_ok());
    let activities = result.unwrap();
    assert_eq!(activities.len(), 1);

    let activity = &activities[0];
    assert_eq!(activity.id.value(), 143592);
    assert_eq!(activity.type_id, 1);
    // In Phase 1, project is stored as serde_json::Value
    assert_eq!(activity.project["name"], "Example Project");
}

#[tokio::test]
async fn test_get_project_recent_updates_with_filters() {
    let server = MockServer::start().await;
    let project_api = setup_project_api(&server).await;

    let mock_response = serde_json::json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/EXAMPLE/activities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let mut params = GetProjectRecentUpdatesParams::new(ProjectKey::from_str("EXAMPLE").unwrap());
    params.activity_type_ids = Some(vec![ActivityTypeId::new(1), ActivityTypeId::new(2)]);
    params.count = Some(50);
    params.order = Some("desc".to_string());

    let result = project_api.get_project_recent_updates(params).await;
    if let Err(ref e) = result {
        println!("Error in filter test: {e:?}");
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_project_recent_updates_with_pagination() {
    let server = MockServer::start().await;
    let project_api = setup_project_api(&server).await;

    let mock_response = serde_json::json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/123/activities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let mut params = GetProjectRecentUpdatesParams::new(ProjectId::new(123));
    params.min_id = Some(ActivityId::new(100));
    params.max_id = Some(ActivityId::new(200));

    let result = project_api.get_project_recent_updates(params).await;
    if let Err(ref e) = result {
        println!("Error in filter test: {e:?}");
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_project_recent_updates_not_found() {
    let server = MockServer::start().await;
    let project_api = setup_project_api(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/NOTFOUND/activities"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let params = GetProjectRecentUpdatesParams::new(ProjectKey::from_str("NOTFOUND").unwrap());
    let result = project_api.get_project_recent_updates(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_project_recent_updates_unauthorized() {
    let server = MockServer::start().await;
    let project_api = setup_project_api(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/projects/EXAMPLE/activities"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;

    let params = GetProjectRecentUpdatesParams::new(ProjectKey::from_str("EXAMPLE").unwrap());
    let result = project_api.get_project_recent_updates(params).await;
    assert!(result.is_err());
}
