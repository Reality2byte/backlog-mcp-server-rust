#[cfg(all(test, feature = "writable"))]
mod custom_field_edge_cases_tests {
    use backlog_core::identifier::{
        CustomFieldId, CustomFieldItemId, IssueTypeId, PriorityId, ProjectId,
    };
    use backlog_issue::api::{AddIssueParamsBuilder, IssueApi};
    use backlog_issue::models::CustomFieldInput;
    use chrono::NaiveDate;
    use client::test_utils::setup_client;
    use std::collections::HashMap;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::*};

    #[tokio::test]
    async fn test_very_long_text_field() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        // Create a very long text (10,000 characters)
        let long_text = "a".repeat(10_000);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Long Text Test",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {
                    "id": 1,
                    "fieldTypeId": 1,
                    "name": "Long Text Field",
                    "value": "truncated..."
                }
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .and(body_string_contains("customField_1="))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(CustomFieldId::new(1), CustomFieldInput::Text(long_text));

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Long Text Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_special_characters_in_text_field() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let special_text = "テスト🎉 <script>alert('xss')</script> & \"quotes\" 'single' \n\r\t";

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Special Characters Test",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {
                    "id": 2,
                    "fieldTypeId": 2,
                    "name": "Special Text Field",
                    "value": "テスト🎉 <script>alert('xss')</script> & \"quotes\" 'single' \n\r\t"
                }
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            // URL encoding will handle special characters
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(2),
            CustomFieldInput::TextArea(special_text.to_string()),
        );

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Special Characters Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
        let issue = result.unwrap();
        assert_eq!(issue.custom_fields.len(), 1);
    }

    #[tokio::test]
    async fn test_numeric_field_extremes() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Numeric Extremes Test",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {"id": 3, "fieldTypeId": 3, "name": "Max Number", "value": 1.7976931348623157e308},
                {"id": 4, "fieldTypeId": 3, "name": "Min Number", "value": -1.7976931348623157e308},
                {"id": 5, "fieldTypeId": 3, "name": "Zero", "value": 0},
                {"id": 6, "fieldTypeId": 3, "name": "Small Decimal", "value": 0.0000000001}
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(CustomFieldId::new(3), CustomFieldInput::Numeric(f64::MAX));
        custom_fields.insert(CustomFieldId::new(4), CustomFieldInput::Numeric(f64::MIN));
        custom_fields.insert(CustomFieldId::new(5), CustomFieldInput::Numeric(0.0));
        custom_fields.insert(
            CustomFieldId::new(6),
            CustomFieldInput::Numeric(0.0000000001),
        );

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Numeric Extremes Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
        let issue = result.unwrap();
        assert_eq!(issue.custom_fields.len(), 4);
    }

    #[tokio::test]
    async fn test_date_field_extremes() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Date Extremes Test",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {"id": 7, "fieldTypeId": 4, "name": "Min Date", "value": "1900-01-01"},
                {"id": 8, "fieldTypeId": 4, "name": "Max Date", "value": "2100-12-31"},
                {"id": 9, "fieldTypeId": 4, "name": "Leap Day", "value": "2024-02-29"}
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(7),
            CustomFieldInput::Date(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
        );
        custom_fields.insert(
            CustomFieldId::new(8),
            CustomFieldInput::Date(NaiveDate::from_ymd_opt(2100, 12, 31).unwrap()),
        );
        custom_fields.insert(
            CustomFieldId::new(9),
            CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()),
        );

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Date Extremes Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_empty_multiple_list_and_checkbox() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Empty Lists Test",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {"id": 10, "fieldTypeId": 6, "name": "Empty Multiple List", "value": []},
                {"id": 11, "fieldTypeId": 7, "name": "Empty Checkbox", "value": []}
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(10),
            CustomFieldInput::MultipleList {
                ids: vec![],
                other_value: None,
            },
        );
        custom_fields.insert(CustomFieldId::new(11), CustomFieldInput::CheckBox(vec![]));

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Empty Lists Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_large_multiple_selection() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        // Create a list with 100 items
        let large_ids: Vec<CustomFieldItemId> = (1..=100).map(CustomFieldItemId::new).collect();

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Large Selection Test",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {"id": 12, "fieldTypeId": 6, "name": "Large Multiple List", "value": []},
                {"id": 13, "fieldTypeId": 7, "name": "Large Checkbox", "value": []}
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(12),
            CustomFieldInput::MultipleList {
                ids: large_ids.clone(),
                other_value: None,
            },
        );
        custom_fields.insert(
            CustomFieldId::new(13),
            CustomFieldInput::CheckBox(large_ids),
        );

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Large Selection Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_other_value_edge_cases() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Other Value Edge Cases",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {"id": 14, "fieldTypeId": 5, "name": "Single with Empty Other", "value": {"id": 100, "name": "Option"}, "otherValue": ""},
                {"id": 15, "fieldTypeId": 8, "name": "Radio with Long Other", "value": {"id": 200, "name": "Option"}, "otherValue": "Very long other value text that exceeds normal expectations..."}
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .and(body_string_contains("customField_14_otherValue="))
            .and(body_string_contains("customField_15_otherValue="))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(14),
            CustomFieldInput::SingleList {
                id: CustomFieldItemId::new(100),
                other_value: Some("".to_string()),
            },
        );
        custom_fields.insert(
            CustomFieldId::new(15),
            CustomFieldInput::Radio {
                id: CustomFieldItemId::new(200),
                other_value: Some(
                    "Very long other value text that exceeds normal expectations...".to_string(),
                ),
            },
        );

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Other Value Edge Cases".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_all_custom_fields_null_values() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        // Test when API returns null values for custom fields
        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Null Custom Fields Test",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [
                {"id": 16, "fieldTypeId": 1, "name": "Null Text", "value": null},
                {"id": 17, "fieldTypeId": 3, "name": "Null Number", "value": null},
                {"id": 18, "fieldTypeId": 4, "name": "Null Date", "value": null}
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Null Custom Fields Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        // This might fail if our deserialization doesn't handle null values properly
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_numeric_field_nan_and_infinity() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Special Numeric Values",
            "description": "",
            "priority": {"id": 2, "name": "Normal"},
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ed8077", "displayOrder": 1000},
            "assignee": null,
            "category": [],
            "versions": [],
            "milestone": [],
            "startDate": null,
            "dueDate": null,
            "estimatedHours": null,
            "actualHours": null,
            "parentIssueId": null,
            "customFields": [],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T10:00:00Z"
        }"##;

        Mock::given(method("POST"))
            .and(path("/api/v2/issues"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        // Test special float values
        custom_fields.insert(
            CustomFieldId::new(19),
            CustomFieldInput::Numeric(f64::INFINITY),
        );
        custom_fields.insert(
            CustomFieldId::new(20),
            CustomFieldInput::Numeric(f64::NEG_INFINITY),
        );
        custom_fields.insert(CustomFieldId::new(21), CustomFieldInput::Numeric(f64::NAN));

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Special Numeric Values".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        // These special values might be rejected by the API
        assert!(result.is_ok());
    }
}
