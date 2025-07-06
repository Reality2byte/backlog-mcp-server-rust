#[cfg(feature = "writable")]
use backlog_api_core::IntoRequest;
#[cfg(feature = "writable")]
use backlog_core::identifier::{Identifier, NotificationId};
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct MarkNotificationAsReadParams {
    notification_id: NotificationId,
}

#[cfg(feature = "writable")]
impl MarkNotificationAsReadParams {
    pub fn new(notification_id: impl Into<NotificationId>) -> Self {
        Self {
            notification_id: notification_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for MarkNotificationAsReadParams {
    fn method(&self) -> backlog_api_core::HttpMethod {
        backlog_api_core::HttpMethod::Post
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/notifications/{}/markAsRead",
            self.notification_id.value()
        )
    }

    fn to_form(&self) -> impl Serialize {
        // No form data needed for this endpoint
        let params: Vec<(String, String)> = Vec::new();
        params
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use crate::api::UserApi;
    use client::test_utils::setup_client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_api(mock_server: &MockServer) -> UserApi {
        let client = setup_client(mock_server).await;
        UserApi::new(client)
    }

    #[tokio::test]
    async fn test_mark_notification_as_read_success() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;
        let notification_id = NotificationId::new(123);

        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/123/markAsRead"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = api.mark_notification_as_read(notification_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mark_notification_as_read_not_found() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;
        let notification_id = NotificationId::new(999);

        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/999/markAsRead"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{
                    "message": "No notification found.",
                    "code": 7,
                    "moreInfo": ""
                }]
            })))
            .mount(&mock_server)
            .await;

        let result = api.mark_notification_as_read(notification_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mark_notification_as_read_unauthorized() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;
        let notification_id = NotificationId::new(123);

        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/123/markAsRead"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "errors": [{
                    "message": "Unauthorized",
                    "code": 11,
                    "moreInfo": ""
                }]
            })))
            .mount(&mock_server)
            .await;

        let result = api.mark_notification_as_read(notification_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mark_notification_as_read_already_read() {
        let mock_server = MockServer::start().await;
        let api = setup_api(&mock_server).await;
        let notification_id = NotificationId::new(456);

        // Even if already read, API should return 204
        Mock::given(method("POST"))
            .and(path("/api/v2/notifications/456/markAsRead"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = api.mark_notification_as_read(notification_id).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_into_request_path() {
        let params = MarkNotificationAsReadParams::new(NotificationId::new(789));
        assert_eq!(params.path(), "/api/v2/notifications/789/markAsRead");
    }
}
