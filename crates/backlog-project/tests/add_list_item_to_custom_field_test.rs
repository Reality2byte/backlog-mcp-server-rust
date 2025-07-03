#[cfg(feature = "writable")]
mod add_list_item_to_custom_field_tests {
    use backlog_core::{
        ProjectKey,
        identifier::{CustomFieldId, Identifier, ProjectId},
    };
    use backlog_project::{AddListItemToCustomFieldParams, ProjectApi};
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_add_list_item_to_single_list_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 5,
            "projectId": 1,
            "typeId": 5,  // Single list type
            "name": "Priority",
            "description": "Custom priority field",
            "required": true,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 1, "name": "Low", "displayOrder": 0},
                {"id": 2, "name": "Medium", "displayOrder": 1},
                {"id": 3, "name": "High", "displayOrder": 2},
                {"id": 4, "name": "Critical", "displayOrder": 3}  // Newly added
            ],
            "allowAddItem": true,
            "displayOrder": 5
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields/5/items"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            "Critical".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 5);
        assert_eq!(custom_field.name, "Priority");
    }

    #[tokio::test]
    async fn test_add_list_item_to_multiple_list_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 6,
            "projectId": 1,
            "typeId": 6,  // Multiple list type
            "name": "Tags",
            "description": "Issue tags",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 1, "name": "bug", "displayOrder": 0},
                {"id": 2, "name": "feature", "displayOrder": 1},
                {"id": 3, "name": "enhancement", "displayOrder": 2}  // Newly added
            ],
            "allowAddItem": true,
            "allowInput": false,
            "displayOrder": 6
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields/6/items"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(6),
            "enhancement".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 6);
        assert_eq!(custom_field.name, "Tags");
    }

    #[tokio::test]
    async fn test_add_list_item_with_japanese_name() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 7,
            "projectId": 1,
            "typeId": 5,
            "name": "重要度",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 1, "name": "低", "displayOrder": 0},
                {"id": 2, "name": "中", "displayOrder": 1},
                {"id": 3, "name": "高", "displayOrder": 2},
                {"id": 4, "name": "緊急", "displayOrder": 3}  // Newly added
            ],
            "allowAddItem": true,
            "displayOrder": 7
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields/7/items"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(7),
            "緊急".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 7);
        assert_eq!(custom_field.name, "重要度");
    }

    #[tokio::test]
    async fn test_add_list_item_to_non_list_field_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields/1/items"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "Custom field type is not list",
                        "code": 3,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = AddListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(1), // Text field
            "Invalid".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_list_item_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields/999/items"))
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

        let params = AddListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(999),
            "New Item".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_list_item_permission_denied() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields/8/items"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "You do not have permission to add list items",
                        "code": 11,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = AddListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(8),
            "New Item".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_list_item_with_project_id() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 10,
            "projectId": 12345,
            "typeId": 5,
            "name": "Status",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 1, "name": "Open", "displayOrder": 0},
                {"id": 2, "name": "In Progress", "displayOrder": 1},
                {"id": 3, "name": "Closed", "displayOrder": 2}
            ],
            "allowAddItem": true,
            "displayOrder": 10
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/12345/customFields/10/items"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddListItemToCustomFieldParams::new(
            ProjectId::new(12345),
            CustomFieldId::new(10),
            "Closed".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 10);
        assert_eq!(custom_field.project_id.value(), 12345);
    }

    #[tokio::test]
    async fn test_add_list_item_empty_name() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields/5/items"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "name is required",
                        "code": 3,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = AddListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            "".to_string(),
        );
        let result = api.add_list_item_to_custom_field(params).await;

        assert!(result.is_err());
    }
}
