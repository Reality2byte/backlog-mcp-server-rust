use serde_json::json;
use wiremock::MockServer;

pub async fn setup_mock_server() -> MockServer {
    MockServer::start().await
}

#[allow(dead_code)]
pub fn mock_webhook_list_response() -> serde_json::Value {
    json!([
        {
            "id": 1,
            "name": "webhook1",
            "description": "test webhook 1",
            "hookUrl": "http://example.com/webhook1",
            "allEvent": false,
            "activityTypeIds": [1, 2, 3, 4, 5],
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "test@example.com"
            },
            "created": "2023-01-01T00:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "test@example.com"
            },
            "updated": "2023-01-01T00:00:00Z"
        },
        {
            "id": 2,
            "name": "webhook2",
            "description": "test webhook 2",
            "hookUrl": "http://example.com/webhook2",
            "allEvent": true,
            "activityTypeIds": [],
            "createdUser": {
                "id": 2,
                "userId": "user1",
                "name": "User 1",
                "roleType": 2,
                "lang": "en",
                "mailAddress": "user1@example.com"
            },
            "created": "2023-01-02T00:00:00Z",
            "updatedUser": {
                "id": 2,
                "userId": "user1",
                "name": "User 1",
                "roleType": 2,
                "lang": "en",
                "mailAddress": "user1@example.com"
            },
            "updated": "2023-01-02T00:00:00Z"
        }
    ])
}

#[allow(dead_code)]
pub fn mock_empty_webhook_list_response() -> serde_json::Value {
    json!([])
}

pub fn mock_error_response() -> serde_json::Value {
    json!({
        "errors": [
            {
                "message": "No project with id or key: INVALID",
                "code": 6,
                "moreInfo": ""
            }
        ]
    })
}

pub fn mock_single_webhook_response() -> serde_json::Value {
    json!({
        "id": 1,
        "name": "webhook1",
        "description": "test webhook 1",
        "hookUrl": "http://example.com/webhook1",
        "allEvent": false,
        "activityTypeIds": [1, 2, 3, 4, 5],
        "createdUser": {
            "id": 1,
            "userId": "admin",
            "name": "admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "test@example.com"
        },
        "created": "2023-01-01T00:00:00Z",
        "updatedUser": {
            "id": 1,
            "userId": "admin",
            "name": "admin",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "test@example.com"
        },
        "updated": "2023-01-01T00:00:00Z"
    })
}
