#[cfg(all(test, feature = "writable"))]
mod tests {
    use backlog_core::{
        ProjectKey,
        identifier::{CustomFieldId, CustomListItemId, Identifier},
    };
    use backlog_project::{ProjectApi, UpdateListItemToCustomFieldParams};
    use client::Client;
    use std::str::FromStr;
    use wiremock::matchers::{body_string_contains, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_client(mock_server: &MockServer) -> Client {
        let base_url = mock_server.uri();
        let api_key = "test_api_key";
        Client::new(&base_url).unwrap().with_api_key(api_key)
    }

    #[tokio::test]
    async fn test_update_list_item_success() {
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
                {"id": 4, "name": "Critical Updated", "displayOrder": 3}  // Updated name
            ],
            "allowAddItem": true,
            "displayOrder": 5
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/5/items/4"))
            .and(body_string_contains("name=Critical+Updated"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            4, // itemId
            "Critical Updated".to_string(),
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 5);
        assert_eq!(custom_field.name, "Priority");

        // Verify the updated item
        if let backlog_issue::CustomFieldSettings::SingleList(settings) = custom_field.settings {
            let updated_item = settings
                .items
                .iter()
                .find(|i| i.id == CustomListItemId::new(4))
                .unwrap();
            assert_eq!(updated_item.name, "Critical Updated");
        } else {
            panic!("Expected SingleList type in response");
        }
    }

    #[tokio::test]
    async fn test_update_list_item_to_multiple_list_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 10,
            "projectId": 1,
            "typeId": 6,  // Multiple list type
            "name": "Tags",
            "description": "Multiple selection tags",
            "required": false,
            "applicableIssueTypes": [1, 2, 3],
            "useIssueType": false,
            "items": [
                {"id": 10, "name": "Backend Updated", "displayOrder": 0},
                {"id": 11, "name": "Frontend", "displayOrder": 1},
                {"id": 12, "name": "Database", "displayOrder": 2}
            ],
            "allowAddItem": true,
            "displayOrder": 10
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/10/items/10"))
            .and(body_string_contains("name=Backend+Updated"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(10),
            10, // itemId
            "Backend Updated".to_string(),
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 10);

        // Verify the updated item
        if let backlog_issue::CustomFieldSettings::MultipleList(settings) = custom_field.settings {
            let updated_item = settings
                .items
                .iter()
                .find(|i| i.id == CustomListItemId::new(10))
                .unwrap();
            assert_eq!(updated_item.name, "Backend Updated");
            assert_eq!(settings.items.len(), 3);
        } else {
            panic!("Expected MultipleList type in response");
        }
    }

    #[tokio::test]
    async fn test_update_list_item_with_special_characters() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 15,
            "projectId": 1,
            "typeId": 5,
            "name": "Environment",
            "description": "",
            "required": true,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 20, "name": "Dev", "displayOrder": 0},
                {"id": 21, "name": "Test/QA & Staging", "displayOrder": 1}  // Updated with special chars
            ],
            "allowAddItem": true,
            "displayOrder": 15
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/15/items/21"))
            .and(body_string_contains("name=Test%2FQA+%26+Staging"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(15),
            21,
            "Test/QA & Staging".to_string(),
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        if let backlog_issue::CustomFieldSettings::SingleList(settings) = custom_field.settings {
            let updated_item = settings
                .items
                .iter()
                .find(|i| i.id == CustomListItemId::new(21))
                .unwrap();
            assert_eq!(updated_item.name, "Test/QA & Staging");
        }
    }

    #[tokio::test]
    async fn test_update_list_item_japanese_characters() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 20,
            "projectId": 1,
            "typeId": 5,
            "name": "地域",
            "description": "対象地域",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 30, "name": "東京・関東エリア", "displayOrder": 0},  // Updated
                {"id": 31, "name": "大阪", "displayOrder": 1}
            ],
            "allowAddItem": true,
            "displayOrder": 20
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/20/items/30"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(20),
            30,
            "東京・関東エリア".to_string(),
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.name, "地域");
        if let backlog_issue::CustomFieldSettings::SingleList(settings) = custom_field.settings {
            let updated_item = settings
                .items
                .iter()
                .find(|i| i.id == CustomListItemId::new(30))
                .unwrap();
            assert_eq!(updated_item.name, "東京・関東エリア");
        }
    }

    #[tokio::test]
    async fn test_update_list_item_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such list item",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/5/items/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            999, // Non-existent item
            "Updated Name".to_string(),
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("404"));
    }

    #[tokio::test]
    async fn test_update_list_item_custom_field_not_list_type() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Custom field type is not list",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/1/items/1"))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(1), // Text type field
            1,
            "Updated".to_string(),
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("400"));
    }

    #[tokio::test]
    async fn test_update_list_item_empty_name() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Name is required",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/5/items/1"))
            .and(body_string_contains("name="))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            1,
            "".to_string(), // Empty name
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_list_item_permission_denied() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "You do not have permission to update custom field items",
                    "code": 11,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST/customFields/5/items/1"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateListItemToCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            1,
            "Updated".to_string(),
        );
        let result = api.update_list_item_to_custom_field(params).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("403"));
    }
}
