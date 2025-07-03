mod common;

#[cfg(test)]
mod activity_api_tests {
    use super::common::setup_activity_api;
    use backlog_activity::GetActivityParams;
    use backlog_api_core::IntoRequest;
    use backlog_core::identifier::{ActivityId, Identifier};
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_activity_success() {
        let mock_server = MockServer::start().await;
        let activity_id = ActivityId::from(12345);

        Mock::given(method("GET"))
            .and(path("/api/v2/activities/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": 12345,
                "project": {
                    "id": 101,
                    "projectKey": "TEST",
                    "name": "Test Project",
                    "chartEnabled": false,
                    "subtaskingEnabled": false,
                    "projectLeaderCanEditProjectLeader": false,
                    "useWikiTreeView": false,
                    "textFormattingRule": "backlog",
                    "archived": false,
                    "displayOrder": 0,
                    "useDevAttributes": true,
                    "useWiki": true,
                    "useFileSharing": true,
                    "useOriginalImageSizeAtWiki": false
                },
                "type": 1,
                "content": {
                    "id": 456,
                    "key_id": 789,
                    "summary": "Test issue",
                    "description": "Test description"
                },
                "notifications": [],
                "createdUser": {
                    "id": 12345,
                    "userId": "testuser",
                    "name": "Test User",
                    "roleType": 2,
                    "lang": "ja",
                    "mailAddress": "test@example.com",
                    "nulabAccount": {
                        "nulabId": "nulabtest",
                        "name": "Test Nulab User",
                        "uniqueId": "unique123"
                    },
                    "keyword": "test keyword"
                },
                "created": "2024-01-01T10:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let api = setup_activity_api(&mock_server).await;
        let result = api.get_activity(activity_id).await;

        assert!(result.is_ok());
        let activity = result.unwrap();
        assert_eq!(activity.id.value(), 12345);
        assert_eq!(activity.project.id.value(), 101);
    }

    #[tokio::test]
    async fn test_get_activity_not_found() {
        let mock_server = MockServer::start().await;
        let activity_id = ActivityId::from(99999);

        Mock::given(method("GET"))
            .and(path("/api/v2/activities/99999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "errors": [
                    {
                        "message": "No activity found",
                        "code": 5,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let api = setup_activity_api(&mock_server).await;
        let result = api.get_activity(activity_id).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_get_activity_path() {
        let activity_id = ActivityId::from(12345);
        let params = GetActivityParams { activity_id };

        assert_eq!(params.path(), "/api/v2/activities/12345");
    }
}
