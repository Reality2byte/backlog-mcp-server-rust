mod common;

use backlog_rate_limit::*;
use client::Client;
use common::*;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_api(server: &MockServer) -> RateLimitApi {
    let client = Client::new(&server.uri())
        .unwrap()
        .with_api_key("test_api_key");
    RateLimitApi::new(client)
}

#[tokio::test]
async fn test_get_rate_limit_success() {
    let server = setup_server().await;
    let api = setup_api(&server).await;
    let expected_response = rate_limit_json();

    Mock::given(method("GET"))
        .and(path("/api/v2/rateLimit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .mount(&server)
        .await;

    let result = api.get_rate_limit().await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.rate_limit.read.limit, 600);
    assert_eq!(response.rate_limit.read.remaining, 598);
    assert_eq!(response.rate_limit.read.reset, 1603881873);

    assert_eq!(response.rate_limit.update.limit, 150);
    assert_eq!(response.rate_limit.update.remaining, 149);
    assert_eq!(response.rate_limit.update.reset, 1603881873);

    assert_eq!(response.rate_limit.search.limit, 150);
    assert_eq!(response.rate_limit.search.remaining, 150);
    assert_eq!(response.rate_limit.search.reset, 1603881873);

    assert_eq!(response.rate_limit.icon.limit, 60);
    assert_eq!(response.rate_limit.icon.remaining, 59);
    assert_eq!(response.rate_limit.icon.reset, 1603881873);
}

#[tokio::test]
async fn test_get_rate_limit_api_error() {
    let server = setup_server().await;
    let api = setup_api(&server).await;
    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "No api key.",
                "code": 1,
                "errorInfo": "problem with the api key",
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/rateLimit"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&server)
        .await;

    let result = api.get_rate_limit().await;
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(e.to_string(), "Backlog API Error (HTTP 401): No api key.");
    }
}

#[tokio::test]
async fn test_get_rate_limit_network_error() {
    // Use an invalid URL that will fail to connect
    let client = Client::new("http://invalid-domain-that-does-not-exist.local")
        .unwrap()
        .with_api_key("test_api_key");
    let api = RateLimitApi::new(client);

    let result = api.get_rate_limit().await;
    assert!(result.is_err());
}
