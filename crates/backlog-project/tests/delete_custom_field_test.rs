#[cfg(feature = "writable")]
mod delete_custom_field_tests {
    use backlog_core::{
        ProjectKey,
        identifier::{CustomFieldId, Identifier, ProjectId},
    };
    use backlog_project::{DeleteCustomFieldParams, ProjectApi};
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_delete_custom_field_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 1,
            "projectId": 1,
            "typeId": 1,
            "name": "説明",
            "description": "課題の詳細な説明",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "displayOrder": 1
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = DeleteCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(1),
        );
        let result = api.delete_custom_field(params).await;

        assert!(result.is_ok());
        let deleted_field = result.unwrap();
        assert_eq!(deleted_field.id.value(), 1);
        assert_eq!(deleted_field.name, "説明");
        assert_eq!(deleted_field.project_id.value(), 1);
    }

    #[tokio::test]
    async fn test_delete_custom_field_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "No such custom field",
                        "code": 5,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(999),
        );
        let result = api.delete_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_custom_field_invalid_project() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/INVALID/customFields/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "No project.",
                        "code": 6,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteCustomFieldParams::new(
            ProjectKey::from_str("INVALID").unwrap(),
            CustomFieldId::new(1),
        );
        let result = api.delete_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_custom_field_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/1"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "You do not have permission to delete custom fields",
                        "code": 11,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(1),
        );
        let result = api.delete_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_custom_field_with_list_type() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 5,
            "projectId": 1,
            "typeId": 5,
            "name": "優先度",
            "description": "カスタム優先度フィールド",
            "required": true,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 1, "name": "低", "displayOrder": 1},
                {"id": 2, "name": "中", "displayOrder": 2},
                {"id": 3, "name": "高", "displayOrder": 3}
            ],
            "allowAddItem": true,
            "displayOrder": 5
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = DeleteCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
        );
        let result = api.delete_custom_field(params).await;

        assert!(result.is_ok());
        let deleted_field = result.unwrap();
        assert_eq!(deleted_field.id.value(), 5);
        assert_eq!(deleted_field.name, "優先度");
    }

    #[tokio::test]
    async fn test_delete_custom_field_with_project_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 10,
            "projectId": 12345,
            "typeId": 3,
            "name": "工数",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "min": 0.0,
            "max": 100.0,
            "initialValue": 0.0,
            "unit": "時間",
            "displayOrder": 10
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/12345/customFields/10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = DeleteCustomFieldParams::new(ProjectId::new(12345), CustomFieldId::new(10));
        let result = api.delete_custom_field(params).await;

        assert!(result.is_ok());
        let deleted_field = result.unwrap();
        assert_eq!(deleted_field.id.value(), 10);
        assert_eq!(deleted_field.name, "工数");
    }
}
