#[cfg(test)]
mod watching_model_tests {
    use serde_json::json;

    use backlog_core::identifier::WatchingId;

    use crate::models::{Watching, WatchingCount, WatchingType};

    #[test]
    fn test_watching_deserialization_full() {
        let json_data = json!({
            "id": 123,
            "resourceAlreadyRead": true,
            "note": "Important issue to track",
            "type": "issue",
            "issue": {
                "id": 456,
                "projectId": 1,
                "issueKey": "PROJ-123",
                "keyId": 123,
                "issueType": {
                    "id": 1,
                    "projectId": 1,
                    "name": "Bug",
                    "color": "#990000",
                    "displayOrder": 0
                },
                "summary": "Fix critical bug",
                "description": "Bug description",
                "resolution": null,
                "priority": {
                    "id": 2,
                    "name": "Normal"
                },
                "status": {
                    "id": 1,
                    "projectId": 1,
                    "name": "Open",
                    "color": "#ed8077",
                    "displayOrder": 1000
                },
                "assignee": null,
                "category": [],
                "versions": [],
                "milestone": [],
                "startDate": null,
                "dueDate": null,
                "estimatedHours": null,
                "actualHours": null,
                "parentIssueId": null,
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "admin",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "test@example.com",
                    "nulabAccount": null,
                    "keyword": "keyword"
                },
                "created": "2022-01-01T00:00:00Z",
                "updatedUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "admin",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "test@example.com",
                    "nulabAccount": null,
                    "keyword": "keyword"
                },
                "updated": "2022-01-02T00:00:00Z",
                "customFields": [],
                "attachments": [],
                "sharedFiles": [],
                "stars": []
            },
            "lastContentUpdated": "2024-01-15T10:00:00Z",
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-15T00:00:00Z",
            "alreadyRead": false
        });

        let watching: Watching = serde_json::from_value(json_data).unwrap();
        assert_eq!(watching.id, WatchingId::from(123));
        assert!(watching.resource_already_read);
        assert_eq!(watching.note, Some("Important issue to track".to_string()));
        assert_eq!(watching.watching_type, WatchingType::Issue);
        assert!(watching.issue.is_some());
        assert!(watching.last_content_updated.is_some());
        assert!(!watching.already_read);
    }

    #[test]
    fn test_watching_deserialization_minimal() {
        let json_data = json!({
            "id": 456,
            "type": "issue",
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-15T00:00:00Z"
        });

        let watching: Watching = serde_json::from_value(json_data).unwrap();
        assert_eq!(watching.id, WatchingId::from(456));
        assert!(!watching.resource_already_read); // default value
        assert_eq!(watching.note, None);
        assert_eq!(watching.watching_type, WatchingType::Issue);
        assert!(watching.issue.is_none());
        assert!(watching.last_content_updated.is_none());
        assert!(!watching.already_read); // default value
    }

    #[test]
    fn test_watching_count_deserialization() {
        let json_data = json!({
            "count": 42
        });

        let count: WatchingCount = serde_json::from_value(json_data).unwrap();
        assert_eq!(count.count, 42);
    }

    #[test]
    fn test_watching_type_serialization() {
        assert_eq!(
            serde_json::to_value(WatchingType::Issue).unwrap(),
            json!("issue")
        );
    }

    #[test]
    fn test_watching_type_deserialization() {
        let watching_type: WatchingType = serde_json::from_value(json!("issue")).unwrap();
        assert_eq!(watching_type, WatchingType::Issue);
    }
}
