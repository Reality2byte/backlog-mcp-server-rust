#[cfg(feature = "writable")]
mod delete_list_item_from_custom_field_tests {
    use backlog_core::{
        ProjectKey,
        identifier::{CustomFieldId, CustomFieldItemId, Identifier, ProjectId},
    };
    use backlog_project::{ProjectApi, api::DeleteListItemFromCustomFieldParams};
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_delete_list_item_from_single_list_success() {
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
                {"id": 3, "name": "High", "displayOrder": 2}
                // Item with id 4 ("Critical") was deleted
            ],
            "allowAddItem": true,
            "displayOrder": 5
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/5/items/4"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = DeleteListItemFromCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            CustomFieldItemId::new(4),
        );
        let result = api.delete_list_item_from_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 5);
        assert_eq!(custom_field.name, "Priority");
        // The response returns the updated custom field definition
    }

    #[tokio::test]
    async fn test_delete_list_item_from_multiple_list_success() {
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
                {"id": 2, "name": "feature", "displayOrder": 1}
                // Item with id 3 ("enhancement") was deleted
            ],
            "allowAddItem": true,
            "allowInput": false,
            "displayOrder": 6
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/6/items/3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = DeleteListItemFromCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(6),
            CustomFieldItemId::new(3),
        );
        let result = api.delete_list_item_from_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 6);
        assert_eq!(custom_field.name, "Tags");
    }

    #[tokio::test]
    async fn test_delete_list_item_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/5/items/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "No such custom field item",
                        "code": 5,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteListItemFromCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            CustomFieldItemId::new(999),
        );
        let result = api.delete_list_item_from_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_list_item_invalid_custom_field() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/999/items/1"))
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

        let params = DeleteListItemFromCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(999),
            CustomFieldItemId::new(1),
        );
        let result = api.delete_list_item_from_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_list_item_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/5/items/1"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "You do not have permission to delete custom field items",
                        "code": 11,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteListItemFromCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(5),
            CustomFieldItemId::new(1),
        );
        let result = api.delete_list_item_from_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_list_item_with_project_id() {
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
                {"id": 2, "name": "In Progress", "displayOrder": 1}
            ],
            "allowAddItem": true,
            "displayOrder": 10
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/12345/customFields/10/items/3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = DeleteListItemFromCustomFieldParams::new(
            ProjectId::new(12345),
            CustomFieldId::new(10),
            CustomFieldItemId::new(3),
        );
        let result = api.delete_list_item_from_custom_field(params).await;

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.id.value(), 10);
        assert_eq!(custom_field.name, "Status");
        // The response contains the updated custom field definition
    }

    #[tokio::test]
    async fn test_delete_list_item_not_list_type() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/TEST/customFields/1/items/1"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "errors": [
                    {
                        "message": "Custom field is not a list type",
                        "code": 7,
                        "moreInfo": ""
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let params = DeleteListItemFromCustomFieldParams::new(
            ProjectKey::from_str("TEST").unwrap(),
            CustomFieldId::new(1), // Not a list type custom field
            CustomFieldItemId::new(1),
        );
        let result = api.delete_list_item_from_custom_field(params).await;

        assert!(result.is_err());
    }
}
