#[cfg(all(test, feature = "writable"))]
mod custom_field_integration_tests {
    use backlog_core::IssueKey;
    use backlog_core::identifier::{CustomFieldId, IssueTypeId, PriorityId, ProjectId};
    use backlog_issue::api::{AddIssueParamsBuilder, IssueApi, UpdateIssueParamsBuilder};
    use backlog_issue::models::CustomFieldInput;
    use chrono::NaiveDate;
    use client::test_utils::setup_client;
    use std::collections::HashMap;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::*};

    #[tokio::test]
    async fn test_add_issue_with_custom_fields() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Test Issue with Custom Fields",
            "description": "Test Description",
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
                    "id": 10,
                    "fieldTypeId": 1,
                    "name": "Text Field",
                    "value": "Custom Text Value"
                },
                {
                    "id": 20,
                    "fieldTypeId": 4,
                    "name": "Date Field",
                    "value": "2024-06-24"
                },
                {
                    "id": 30,
                    "fieldTypeId": 5,
                    "name": "Single List",
                    "value": {"id": 123, "name": "Option A"},
                    "otherValue": "Other description"
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
            .and(body_string_contains("customField_10=Custom+Text+Value"))
            .and(body_string_contains("customField_20=2024-06-24"))
            .and(body_string_contains("customField_30=123"))
            .and(body_string_contains(
                "customField_30_otherValue=Other+description",
            ))
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
            CustomFieldInput::Text("Custom Text Value".to_string()),
        );
        custom_fields.insert(
            CustomFieldId::new(20),
            CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap()),
        );
        custom_fields.insert(
            CustomFieldId::new(30),
            CustomFieldInput::SingleList {
                id: 123,
                other_value: Some("Other description".to_string()),
            },
        );

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Test Issue with Custom Fields".to_string())
            .description("Test Description".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
        let issue = result.unwrap();
        assert_eq!(issue.summary, "Test Issue with Custom Fields");
        assert_eq!(issue.custom_fields.len(), 3);
    }

    #[tokio::test]
    async fn test_update_issue_with_custom_fields() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "Updated Issue",
            "description": "Updated Description",
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
                    "id": 40,
                    "fieldTypeId": 6,
                    "name": "Multiple List",
                    "value": [
                        {"id": 100, "name": "Option 1"},
                        {"id": 200, "name": "Option 2"}
                    ]
                },
                {
                    "id": 50,
                    "fieldTypeId": 7,
                    "name": "Checkbox",
                    "value": [
                        {"id": 10, "name": "Check 1"},
                        {"id": 20, "name": "Check 2"},
                        {"id": 30, "name": "Check 3"}
                    ]
                }
            ],
            "attachments": [],
            "sharedFiles": [],
            "stars": [],
            "createdUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "created": "2024-06-24T10:00:00Z",
            "updatedUser": {"id": 1, "userId": "admin", "name": "admin", "roleType": 1, "lang": "ja", "mailAddress": "admin@example.com"},
            "updated": "2024-06-24T11:00:00Z"
        }"##;

        Mock::given(method("PATCH"))
            .and(path("/api/v2/issues/TEST-1"))
            .and(body_string_contains("customField_40=100"))
            .and(body_string_contains("customField_40=200"))
            .and(body_string_contains("customField_50=10"))
            .and(body_string_contains("customField_50=20"))
            .and(body_string_contains("customField_50=30"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(40),
            CustomFieldInput::MultipleList {
                ids: vec![100, 200],
                other_value: None,
            },
        );
        custom_fields.insert(
            CustomFieldId::new(50),
            CustomFieldInput::CheckBox(vec![10, 20, 30]),
        );

        let params = UpdateIssueParamsBuilder::default()
            .issue_id_or_key("TEST-1".parse::<IssueKey>().unwrap())
            .summary("Updated Issue")
            .description("Updated Description")
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        let result = api.update_issue(params).await;
        assert!(result.is_ok());
        let issue = result.unwrap();
        assert_eq!(issue.summary, "Updated Issue");
        assert_eq!(issue.custom_fields.len(), 2);
    }

    #[tokio::test]
    async fn test_add_issue_with_all_custom_field_types() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "All Field Types",
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
                {"id": 1, "fieldTypeId": 1, "name": "Text", "value": "Text Value"},
                {"id": 2, "fieldTypeId": 2, "name": "TextArea", "value": "Multi\nLine\nText"},
                {"id": 3, "fieldTypeId": 3, "name": "Numeric", "value": 123.45},
                {"id": 4, "fieldTypeId": 4, "name": "Date", "value": "2024-12-31"},
                {"id": 5, "fieldTypeId": 5, "name": "SingleList", "value": {"id": 100, "name": "Single"}},
                {"id": 6, "fieldTypeId": 6, "name": "MultipleList", "value": [{"id": 200, "name": "Multi1"}, {"id": 201, "name": "Multi2"}]},
                {"id": 7, "fieldTypeId": 7, "name": "CheckBox", "value": [{"id": 300, "name": "Check"}]},
                {"id": 8, "fieldTypeId": 8, "name": "Radio", "value": {"id": 400, "name": "Radio"}}
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
            .and(body_string_contains("customField_1=Text+Value"))
            .and(body_string_contains("customField_2=Multi%0ALine%0AText"))
            .and(body_string_contains("customField_3=123.45"))
            .and(body_string_contains("customField_4=2024-12-31"))
            .and(body_string_contains("customField_5=100"))
            .and(body_string_contains("customField_6=200"))
            .and(body_string_contains("customField_6=201"))
            .and(body_string_contains("customField_7=300"))
            .and(body_string_contains("customField_8=400"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(
                    serde_json::from_str::<serde_json::Value>(response_json).unwrap(),
                ),
            )
            .mount(&mock_server)
            .await;

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("All Field Types".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_field(
                CustomFieldId::new(1),
                CustomFieldInput::Text("Text Value".to_string()),
            )
            .custom_field(
                CustomFieldId::new(2),
                CustomFieldInput::TextArea("Multi\nLine\nText".to_string()),
            )
            .custom_field(CustomFieldId::new(3), CustomFieldInput::Numeric(123.45))
            .custom_field(
                CustomFieldId::new(4),
                CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
            )
            .custom_field(
                CustomFieldId::new(5),
                CustomFieldInput::SingleList {
                    id: 100,
                    other_value: None,
                },
            )
            .custom_field(
                CustomFieldId::new(6),
                CustomFieldInput::MultipleList {
                    ids: vec![200, 201],
                    other_value: None,
                },
            )
            .custom_field(CustomFieldId::new(7), CustomFieldInput::CheckBox(vec![300]))
            .custom_field(
                CustomFieldId::new(8),
                CustomFieldInput::Radio {
                    id: 400,
                    other_value: None,
                },
            )
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
        let issue = result.unwrap();
        assert_eq!(issue.custom_fields.len(), 8);
    }

    #[tokio::test]
    async fn test_add_issue_empty_custom_fields() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = IssueApi::new(client);

        let response_json = r##"{
            "id": 1,
            "projectId": 1,
            "issueKey": "TEST-1",
            "keyId": 1,
            "issueType": {"id": 1, "projectId": 1, "name": "Bug", "color": "#990000", "displayOrder": 0},
            "summary": "No Custom Fields",
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

        // Test with None custom_fields
        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("No Custom Fields".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .build()
            .unwrap();

        let result = api.add_issue(params).await;
        assert!(result.is_ok());
        let issue = result.unwrap();
        assert_eq!(issue.custom_fields.len(), 0);
    }
}
