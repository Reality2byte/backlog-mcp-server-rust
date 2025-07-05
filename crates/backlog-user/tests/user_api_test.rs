mod common;

use backlog_core::ApiDate;
use backlog_core::identifier::{Identifier, UserId};
use backlog_user::api::{
    GetOwnUserParams, GetUserIconParams, GetUserListParams, GetUserParams, GetUserStarCountParams,
    GetUserStarsParams, StarOrder,
};
use chrono::{DateTime, NaiveDate, Utc};
use common::*;
use wiremock::matchers::{method, path, query_param};
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

#[tokio::test]
async fn test_get_user_star_count_success() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(123);

    let expected_response = serde_json::json!({
        "count": 54
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/users/123/stars/count"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetUserStarCountParams::new(user_id);
    let result = api.get_user_star_count(params).await;
    assert!(result.is_ok());
    let star_count = result.unwrap();
    assert_eq!(star_count.count, 54);
}

#[tokio::test]
async fn test_get_user_star_count_with_date_range() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(123);

    let expected_response = serde_json::json!({
        "count": 10
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/users/123/stars/count"))
        .and(query_param("since", "2024-01-01"))
        .and(query_param("until", "2024-12-31"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let since_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let since_datetime =
        DateTime::<Utc>::from_naive_utc_and_offset(since_date.and_hms_opt(0, 0, 0).unwrap(), Utc);
    let since = ApiDate::from(since_datetime);

    let until_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let until_datetime =
        DateTime::<Utc>::from_naive_utc_and_offset(until_date.and_hms_opt(0, 0, 0).unwrap(), Utc);
    let until = ApiDate::from(until_datetime);

    let params = GetUserStarCountParams::new(user_id)
        .with_since(since)
        .with_until(until);

    let result = api.get_user_star_count(params).await;
    assert!(result.is_ok());
    let star_count = result.unwrap();
    assert_eq!(star_count.count, 10);
}

#[tokio::test]
async fn test_get_user_star_count_zero_stars() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(456);

    let expected_response = serde_json::json!({
        "count": 0
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/users/456/stars/count"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetUserStarCountParams::new(user_id);
    let result = api.get_user_star_count(params).await;
    assert!(result.is_ok());
    let star_count = result.unwrap();
    assert_eq!(star_count.count, 0);
}

#[tokio::test]
async fn test_get_user_stars_success() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(123);

    let expected_response = serde_json::json!([
        {
            "id": 75,
            "comment": null,
            "url": "https://xx.backlog.jp/view/BLG-1",
            "title": "[BLG-1] first issue | 課題の表示 - Backlog",
            "presenter": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com",
                "lastLoginTime": "2024-01-01T00:00:00Z"
            },
            "created": "2024-06-01T10:00:00Z"
        },
        {
            "id": 80,
            "comment": "Great work!",
            "url": "https://xx.backlog.jp/view/BLG-2",
            "title": "[BLG-2] second issue",
            "presenter": {
                "id": 2,
                "userId": "user1",
                "name": "Test User",
                "roleType": 2,
                "lang": "en",
                "mailAddress": "user@example.com",
                "lastLoginTime": null
            },
            "created": "2024-06-02T15:30:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/123/stars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetUserStarsParams::new(user_id);
    let result = api.get_user_stars(params).await;
    assert!(result.is_ok());
    let stars = result.unwrap();
    assert_eq!(stars.len(), 2);
    assert_eq!(stars[0].id, 75);
    assert_eq!(stars[0].comment, None);
    assert_eq!(stars[0].url, "https://xx.backlog.jp/view/BLG-1");
    assert_eq!(stars[0].presenter.name, "admin");
    assert_eq!(stars[1].id, 80);
    assert_eq!(stars[1].comment, Some("Great work!".to_string()));
}

#[tokio::test]
async fn test_get_user_stars_with_pagination() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(123);

    let expected_response = serde_json::json!([
        {
            "id": 150,
            "comment": null,
            "url": "https://xx.backlog.jp/view/TEST-150",
            "title": "Test Issue 150",
            "presenter": {
                "id": 3,
                "userId": "user3",
                "name": "User 3",
                "roleType": 2,
                "lang": "ja",
                "mailAddress": "user3@example.com",
                "lastLoginTime": "2024-01-01T00:00:00Z"
            },
            "created": "2024-06-10T10:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/123/stars"))
        .and(query_param("minId", "100"))
        .and(query_param("maxId", "200"))
        .and(query_param("count", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetUserStarsParams::new(user_id)
        .with_min_id(100)
        .with_max_id(200)
        .with_count(25);

    let result = api.get_user_stars(params).await;
    assert!(result.is_ok());
    let stars = result.unwrap();
    assert_eq!(stars.len(), 1);
    assert_eq!(stars[0].id, 150);
}

#[tokio::test]
async fn test_get_user_stars_with_order() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(123);

    let expected_response = serde_json::json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/123/stars"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetUserStarsParams::new(user_id).with_order(StarOrder::Asc);

    let result = api.get_user_stars(params).await;
    assert!(result.is_ok());
    let stars = result.unwrap();
    assert_eq!(stars.len(), 0);
}

#[tokio::test]
async fn test_get_user_stars_empty_response() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(999);

    let expected_response = serde_json::json!([]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/999/stars"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&mock_server)
        .await;

    let params = GetUserStarsParams::new(user_id);
    let result = api.get_user_stars(params).await;
    assert!(result.is_ok());
    let stars = result.unwrap();
    assert_eq!(stars.len(), 0);
}

#[tokio::test]
async fn test_get_user_stars_not_found() {
    let mock_server = MockServer::start().await;
    let api = setup_user_api(&mock_server).await;
    let user_id = UserId::new(404);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/404/stars"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "errors": [{
                "message": "User not found",
                "code": 1,
                "moreInfo": ""
            }]
        })))
        .mount(&mock_server)
        .await;

    let params = GetUserStarsParams::new(user_id);
    let result = api.get_user_stars(params).await;
    assert!(result.is_err());
}
