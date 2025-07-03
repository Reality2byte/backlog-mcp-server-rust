mod common;

use backlog_core::identifier::{Identifier, UserId};
use backlog_user::api::{GetOwnUserParams, GetUserIconParams, GetUserListParams, GetUserParams};
use common::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_user_success() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(123);

    let expected_user = serde_json::json!({
        "id": 123,
        "userId": "testuser",
        "name": "Test User",
        "roleType": 2,
        "lang": "ja",
        "mailAddress": "test@example.com",
        "lastLoginTime": "2024-06-20T06:35:39Z"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/users/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
        .mount(&mock_server)
        .await;

    let params = GetUserParams::new(user_id);
    let result = api.get_user(params).await;
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.id.value(), 123);
    assert_eq!(user.user_id, Some("testuser".to_string()));
    assert_eq!(user.name, "Test User");
    assert_eq!(user.mail_address, "test@example.com");
}

#[tokio::test]
async fn test_get_user_not_found() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(999);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "errors": [{"message": "User not found"}]
        })))
        .mount(&mock_server)
        .await;

    let result = api.get_user(GetUserParams::new(user_id)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_user_list_success() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;

    let expected_users = serde_json::json!([
        {
            "id": 123,
            "userId": "testuser1",
            "name": "Test User 1",
            "roleType": 2,
            "lang": "ja",
            "mailAddress": "test1@example.com",
            "lastLoginTime": "2024-06-20T06:35:39Z"
        },
        {
            "id": 124,
            "userId": "testuser2",
            "name": "Test User 2",
            "roleType": 1,
            "lang": "en",
            "mailAddress": "test2@example.com",
            "lastLoginTime": "2024-06-21T06:35:39Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
        .mount(&mock_server)
        .await;

    let params = GetUserListParams::new();
    let result = api.get_user_list(params).await;
    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].id.value(), 123);
    assert_eq!(users[1].id.value(), 124);
}

#[tokio::test]
async fn test_get_own_user_success() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;

    let expected_user = serde_json::json!({
        "id": 123,
        "userId": "myself",
        "name": "My User",
        "roleType": 1,
        "lang": "en",
        "mailAddress": "myself@example.com",
        "lastLoginTime": "2024-06-20T06:35:39Z"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/users/myself"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
        .mount(&mock_server)
        .await;

    let params = GetOwnUserParams::new();
    let result = api.get_own_user(params).await;
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.id.value(), 123);
    assert_eq!(user.user_id, Some("myself".to_string()));
    assert_eq!(user.name, "My User");
    assert_eq!(user.mail_address, "myself@example.com");
}

#[tokio::test]
async fn test_get_user_icon_success() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(123);

    let icon_data = b"fake_icon_data";
    Mock::given(method("GET"))
        .and(path("/api/v2/users/123/icon"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(icon_data.as_slice())
                .insert_header("content-type", "image/png")
                .insert_header("content-disposition", "attachment; filename=\"icon.png\""),
        )
        .mount(&mock_server)
        .await;

    let params = GetUserIconParams::new(user_id);
    let result = api.get_user_icon(params).await;
    assert!(result.is_ok());
    let downloaded_file = result.unwrap();
    assert_eq!(downloaded_file.filename, "icon.png");
    assert_eq!(downloaded_file.content_type, "image/png");
    assert_eq!(downloaded_file.bytes.as_ref(), icon_data);
}
