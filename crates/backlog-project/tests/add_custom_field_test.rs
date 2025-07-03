#[cfg(feature = "writable")]
mod add_custom_field_tests {
    use backlog_core::{
        Date, ProjectKey,
        identifier::{Identifier, IssueTypeId},
    };
    use backlog_issue::CustomFieldSettings;
    use backlog_project::{AddCustomFieldParams, ProjectApi};
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_add_text_custom_field() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 1,
            "projectId": 1,
            "typeId": 1,
            "name": "説明",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "displayOrder": 1
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddCustomFieldParams::text(ProjectKey::from_str("TEST").unwrap(), "説明");
        let result = api.add_custom_field(params).await;

        let field = match result {
            Ok(field) => field,
            Err(e) => panic!("Failed to add custom field: {e:?}"),
        };
        assert_eq!(field.id.value(), 1);
        assert_eq!(field.name, "説明");
    }

    #[tokio::test]
    async fn test_add_numeric_custom_field_with_constraints() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 2,
            "projectId": 1,
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
            "displayOrder": 2
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddCustomFieldParams::numeric(ProjectKey::from_str("TEST").unwrap(), "工数")
            .with_numeric_settings(Some(0.0), Some(100.0), Some(0.0), Some("時間".to_string()));
        let result = api.add_custom_field(params).await;

        assert!(result.is_ok());
        let field = result.unwrap();
        assert_eq!(field.name, "工数");
        if let CustomFieldSettings::Numeric(settings) = &field.settings {
            assert_eq!(settings.min, Some(0.0));
            assert_eq!(settings.max, Some(100.0));
            assert_eq!(settings.initial_value, Some(0.0));
            assert_eq!(settings.unit, Some("時間".to_string()));
        } else {
            panic!("Expected numeric settings");
        }
    }

    #[tokio::test]
    async fn test_add_date_custom_field_with_range() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let min_date = Date::from_str("2024-01-01").unwrap();
        let max_date = Date::from_str("2025-12-31").unwrap();

        let expected_response = serde_json::json!({
            "id": 3,
            "projectId": 1,
            "typeId": 4,
            "name": "予定日",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "min": "2024-01-01",
            "max": "2025-12-31",
            "initialValueType": 1,
            "displayOrder": 3
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddCustomFieldParams::date(ProjectKey::from_str("TEST").unwrap(), "予定日")
            .with_date_settings(Some(min_date), Some(max_date), Some(1), None, None);
        let result = api.add_custom_field(params).await;

        assert!(result.is_ok());
        let field = result.unwrap();
        assert_eq!(field.name, "予定日");
    }

    #[tokio::test]
    async fn test_add_single_list_custom_field() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 4,
            "projectId": 1,
            "typeId": 5,
            "name": "優先度",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "items": [
                {"id": 1, "name": "低", "displayOrder": 1},
                {"id": 2, "name": "中", "displayOrder": 2},
                {"id": 3, "name": "高", "displayOrder": 3}
            ],
            "allowAddItem": true,
            "displayOrder": 4
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddCustomFieldParams::single_list(
            ProjectKey::from_str("TEST").unwrap(),
            "優先度",
            vec!["低".to_string(), "中".to_string(), "高".to_string()],
        )
        .with_allow_add_item(true);
        let result = api.add_custom_field(params).await;

        assert!(result.is_ok());
        let field = result.unwrap();
        assert_eq!(field.name, "優先度");
    }

    #[tokio::test]
    async fn test_add_custom_field_with_full_options() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 5,
            "projectId": 1,
            "typeId": 7,
            "name": "確認項目",
            "description": "リリース前のチェックリスト",
            "required": true,
            "applicableIssueTypes": [1, 2],
            "useIssueType": true,
            "displayOrder": 5
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params =
            AddCustomFieldParams::checkbox(ProjectKey::from_str("TEST").unwrap(), "確認項目")
                .with_description("リリース前のチェックリスト")
                .with_required(true)
                .with_applicable_issue_types(vec![IssueTypeId::new(1), IssueTypeId::new(2)]);
        let result = api.add_custom_field(params).await;

        assert!(result.is_ok());
        let field = result.unwrap();
        assert_eq!(field.name, "確認項目");
        assert_eq!(field.description, "リリース前のチェックリスト");
        assert!(field.required);
    }

    #[tokio::test]
    async fn test_add_custom_field_error_invalid_project() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/INVALID/customFields"))
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

        let params = AddCustomFieldParams::text(ProjectKey::from_str("INVALID").unwrap(), "Test");
        let result = api.add_custom_field(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_custom_field_with_project_key() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 6,
            "projectId": 123,
            "typeId": 2,
            "name": "詳細説明",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "displayOrder": 6
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/MFP/customFields"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params =
            AddCustomFieldParams::textarea(ProjectKey::from_str("MFP").unwrap(), "詳細説明");
        let result = api.add_custom_field(params).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_radio_custom_field() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 7,
            "projectId": 1,
            "typeId": 8,
            "name": "環境",
            "description": "",
            "required": false,
            "applicableIssueTypes": [],
            "useIssueType": false,
            "displayOrder": 7
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/TEST/customFields"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AddCustomFieldParams::radio(ProjectKey::from_str("TEST").unwrap(), "環境");
        let result = api.add_custom_field(params).await;

        assert!(result.is_ok());
    }
}
