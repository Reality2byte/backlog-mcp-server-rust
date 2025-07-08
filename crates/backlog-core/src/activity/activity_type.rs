use crate::User;
use crate::identifier::ActivityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Content, EmptyNotification};

#[cfg(feature = "typed-activity")]
use super::project::ActivityProject;
#[cfg(feature = "typed-activity")]
use crate::identifier::Identifier;

/// Unified activity structure that supports all activity contexts
#[cfg(not(feature = "typed-activity"))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: ActivityId,
    pub project: serde_json::Value, // Phase 1: JSON value to avoid circular dependencies
    #[serde(rename = "type")]
    pub type_id: i32,
    pub content: Content,
    pub notifications: Vec<EmptyNotification>,
    pub created_user: User,
    pub created: DateTime<Utc>,
}

/// Unified activity structure with typed project
#[cfg(feature = "typed-activity")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: ActivityId,
    pub project: ActivityProject, // Phase 2: Typed project
    #[serde(rename = "type")]
    pub type_id: i32,
    pub content: Content,
    pub notifications: Vec<EmptyNotification>,
    pub created_user: User,
    pub created: DateTime<Utc>,
}

#[cfg(not(feature = "typed-activity"))]
impl Activity {
    /// Helper method for migration: extract project info from JSON
    pub fn project_id(&self) -> Option<u32> {
        self.project
            .get("id")
            .and_then(|v| v.as_u64())
            .map(|id| id as u32)
    }

    pub fn project_name(&self) -> Option<&str> {
        self.project.get("name").and_then(|v| v.as_str())
    }
}

#[cfg(feature = "typed-activity")]
impl Activity {
    /// Helper method for typed access - returns Option for consistency
    pub fn project_id(&self) -> Option<u32> {
        Some(self.project.id.value())
    }

    pub fn project_name(&self) -> Option<&str> {
        Some(&self.project.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::Identifier;

    #[test]
    fn test_activity_serialization() {
        #[cfg(not(feature = "typed-activity"))]
        let project = serde_json::json!({"id": 1, "name": "Test Project"});

        #[cfg(feature = "typed-activity")]
        let project = {
            use crate::activity::ActivityProject;
            use crate::identifier::ProjectId;
            ActivityProject {
                id: ProjectId::new(1),
                project_key: "TEST".to_string(),
                name: "Test Project".to_string(),
                archived: None,
            }
        };

        let activity = Activity {
            id: ActivityId::new(12345),
            project,
            type_id: 1,
            content: Content::Standard {
                id: 100,
                key_id: Some(200),
                summary: Some("Test Summary".to_string()),
                description: Some("Test Description".to_string()),
                comment: None,
                changes: None,
            },
            notifications: vec![],
            created_user: User {
                id: crate::identifier::UserId::new(1),
                user_id: Some("testuser".to_string()),
                name: "Test User".to_string(),
                role_type: crate::Role::Admin,
                lang: None,
                mail_address: "test@example.com".to_string(),
                last_login_time: None,
            },
            created: DateTime::parse_from_rfc3339("2024-01-01T10:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        };

        let json = serde_json::to_string(&activity).unwrap();
        assert!(json.contains("\"id\":12345"));
        assert!(json.contains("\"type\":1"));
        assert!(json.contains("\"createdUser\""));
        assert!(json.contains("\"created\":\"2024-01-01T10:00:00Z\""));
    }

    #[test]
    fn test_activity_deserialization() {
        #[cfg(not(feature = "typed-activity"))]
        let json = r#"{
            "id": 67890,
            "project": {"id": 2, "name": "Another Project"},
            "type": 2,
            "content": {
                "id": 300,
                "keyId": 400,
                "summary": "Issue Updated",
                "description": "Description updated",
                "comment": {
                    "id": 500,
                    "content": "Update comment"
                },
                "changes": [{
                    "field": "status",
                    "newValue": "Closed",
                    "oldValue": "Open",
                    "type": "standard"
                }]
            },
            "notifications": [],
            "createdUser": {
                "id": 2,
                "userId": "admin",
                "name": "Administrator",
                "roleType": 1,
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-02T15:30:00Z"
        }"#;

        #[cfg(feature = "typed-activity")]
        let json = r#"{
            "id": 67890,
            "project": {"id": 2, "projectKey": "PROJ", "name": "Another Project"},
            "type": 2,
            "content": {
                "id": 300,
                "keyId": 400,
                "summary": "Issue Updated",
                "description": "Description updated",
                "comment": {
                    "id": 500,
                    "content": "Update comment"
                },
                "changes": [{
                    "field": "status",
                    "newValue": "Closed",
                    "oldValue": "Open",
                    "type": "standard"
                }]
            },
            "notifications": [],
            "createdUser": {
                "id": 2,
                "userId": "admin",
                "name": "Administrator",
                "roleType": 1,
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-02T15:30:00Z"
        }"#;

        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.id.value(), 67890);
        assert_eq!(activity.type_id, 2);

        match &activity.content {
            Content::Standard {
                id,
                key_id,
                summary,
                comment,
                changes,
                ..
            } => {
                assert_eq!(*id, 300);
                assert_eq!(*key_id, Some(400));
                assert_eq!(summary.as_deref(), Some("Issue Updated"));
                assert!(comment.is_some());
                assert!(changes.is_some());
                assert_eq!(changes.as_ref().unwrap().len(), 1);
            }
            _ => panic!("Expected Standard content"),
        }
    }

    #[test]
    fn test_activity_with_user_management_content() {
        #[cfg(not(feature = "typed-activity"))]
        let json = r#"{
            "id": 11111,
            "project": {"id": 3},
            "type": 6,
            "content": {
                "users": [{"id": 10, "userId": "newuser", "name": "New User", "roleType": 2, "mailAddress": "newuser@example.com"}],
                "groupProjectActivities": [{"id": 20, "type": 5}],
                "comment": "User added to project"
            },
            "notifications": [],
            "createdUser": {
                "id": 1,
                "name": "Admin",
                "roleType": 1,
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-03T12:00:00Z"
        }"#;

        #[cfg(feature = "typed-activity")]
        let json = r#"{
            "id": 11111,
            "project": {"id": 3, "projectKey": "USER", "name": "User Project"},
            "type": 6,
            "content": {
                "users": [{"id": 10, "userId": "newuser", "name": "New User", "roleType": 2, "mailAddress": "newuser@example.com"}],
                "groupProjectActivities": [{"id": 20, "type": 5}],
                "comment": "User added to project"
            },
            "notifications": [],
            "createdUser": {
                "id": 1,
                "name": "Admin",
                "roleType": 1,
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-03T12:00:00Z"
        }"#;

        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.id.value(), 11111);
        assert_eq!(activity.type_id, 6);

        match &activity.content {
            Content::UserManagement {
                users,
                group_project_activities,
                comment,
            } => {
                assert!(users.is_some());
                assert_eq!(users.as_ref().unwrap().len(), 1);
                assert!(group_project_activities.is_some());
                assert_eq!(comment.as_deref(), Some("User added to project"));
            }
            _ => panic!("Expected UserManagement content"),
        }
    }
}
