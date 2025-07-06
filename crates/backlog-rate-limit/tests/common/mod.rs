use wiremock::MockServer;

pub async fn setup_server() -> MockServer {
    MockServer::start().await
}

pub fn rate_limit_json() -> serde_json::Value {
    serde_json::json!({
        "rateLimit": {
            "read": {
                "limit": 600,
                "remaining": 598,
                "reset": 1603881873
            },
            "update": {
                "limit": 150,
                "remaining": 149,
                "reset": 1603881873
            },
            "search": {
                "limit": 150,
                "remaining": 150,
                "reset": 1603881873
            },
            "icon": {
                "limit": 60,
                "remaining": 59,
                "reset": 1603881873
            }
        }
    })
}
