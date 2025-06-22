mod common;

use backlog_space::api::{GetSpaceLogoParams, GetSpaceParams};
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
