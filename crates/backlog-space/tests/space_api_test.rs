mod common;

use backlog_core::identifier::Identifier;
use backlog_space::api::{GetSpaceDiskUsageParams, GetSpaceLogoParams, GetSpaceParams};
use common::*;
use wiremock::MockServer;

#[tokio::test]
async fn test_get_space_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "spaceKey": "MYSPACE",
        "name": "My Space",
        "ownerId": 1,
        "lang": "ja",
        "timezone": "Asia/Tokyo",
        "reportSendTime": "09:00",
        "textFormattingRule": "markdown",
        "created": "2024-01-01T00:00:00Z",
        "updated": "2024-01-01T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api.get_space(GetSpaceParams::new()).await;
    assert!(result.is_ok());
    let space = result.unwrap();
    assert_eq!(space.name, "My Space");
}

#[tokio::test]
async fn test_get_space_logo_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let logo_content = b"fake_logo_content";

    Mock::given(method("GET"))
        .and(path("/api/v2/space/image"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(logo_content)
                .insert_header("Content-Type", "image/png")
                .insert_header("Content-Disposition", "attachment; filename=\"logo.png\""),
        )
        .mount(&server)
        .await;

    let result = space_api.get_space_logo(GetSpaceLogoParams::new()).await;
    assert!(result.is_ok());
    let downloaded_file = result.unwrap();
    assert_eq!(downloaded_file.filename, "logo.png");
    assert_eq!(downloaded_file.content_type, "image/png");
    assert_eq!(
        downloaded_file.bytes,
        backlog_api_core::bytes::Bytes::from(logo_content.as_slice())
    );
}

#[tokio::test]
async fn test_get_space_disk_usage_success() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "capacity": 10737418240i64, // 10GB
        "issue": 1073741824i64, // 1GB
        "wiki": 536870912i64, // 512MB
        "file": 268435456i64, // 256MB
        "subversion": 0i64,
        "git": 134217728i64, // 128MB
        "gitLFS": 67108864i64, // 64MB
        "details": [
            {
                "projectId": 1,
                "issue": 536870912i64, // 512MB
                "wiki": 268435456i64, // 256MB
                "document": 134217728i64, // 128MB
                "file": 134217728i64, // 128MB
                "subversion": 0i64,
                "git": 67108864i64, // 64MB
                "gitLFS": 33554432i64 // 32MB
            },
            {
                "projectId": 2,
                "issue": 536870912i64, // 512MB
                "wiki": 268435456i64, // 256MB
                "document": 0i64,
                "file": 134217728i64, // 128MB
                "subversion": 0i64,
                "git": 67108864i64, // 64MB
                "gitLFS": 33554432i64 // 32MB
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/diskUsage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_disk_usage(GetSpaceDiskUsageParams::new())
        .await;
    assert!(result.is_ok());
    let disk_usage = result.unwrap();
    assert_eq!(disk_usage.capacity, 10737418240);
    assert_eq!(disk_usage.issue, 1073741824);
    assert_eq!(disk_usage.details.len(), 2);
    assert_eq!(disk_usage.details[0].project_id.value(), 1);
    assert_eq!(disk_usage.details[0].issue, 536870912);
}

#[tokio::test]
async fn test_get_space_disk_usage_forbidden() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "Administrator permissions required",
                "code": 11,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/diskUsage"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_disk_usage(GetSpaceDiskUsageParams::new())
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_space_disk_usage_with_negative_values() {
    let server = MockServer::start().await;
    let space_api = setup_space_api(&server).await;

    let mock_response = serde_json::json!({
        "capacity": 10737418240i64, // 10GB
        "issue": -2610477i64, // Negative value as reported in the issue
        "wiki": 536870912i64, // 512MB
        "file": 268435456i64, // 256MB
        "subversion": 0i64,
        "git": 134217728i64, // 128MB
        "gitLFS": 67108864i64, // 64MB
        "details": [
            {
                "projectId": 1,
                "issue": -1000000i64, // Negative value
                "wiki": 268435456i64,
                "document": 134217728i64,
                "file": 134217728i64,
                "subversion": 0i64,
                "git": 67108864i64,
                "gitLFS": 33554432i64
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/space/diskUsage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    let result = space_api
        .get_space_disk_usage(GetSpaceDiskUsageParams::new())
        .await;
    assert!(result.is_ok());
    let disk_usage = result.unwrap();
    assert_eq!(disk_usage.issue, -2610477);
    assert_eq!(disk_usage.details[0].issue, -1000000);
}
