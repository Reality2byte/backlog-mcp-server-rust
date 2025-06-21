use backlog_core::{User, identifier::WikiAttachmentId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct WikiAttachment {
    pub id: WikiAttachmentId,
    pub name: String,
    pub size: u64,
    #[serde(rename = "createdUser")]
    pub created_user: User,
    pub created: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::{Language, Role, identifier::Identifier};
    use chrono::TimeZone;

    fn create_mock_user() -> User {
        User {
            id: backlog_core::identifier::UserId::new(123),
            user_id: Some("testuser".to_string()),
            name: "Test User".to_string(),
            role_type: Role::User,
            lang: Some(Language::English),
            mail_address: "test@example.com".to_string(),
            last_login_time: Some(Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap()),
        }
    }

    #[test]
    fn test_wiki_attachment_serialization() {
        let attachment = WikiAttachment {
            id: WikiAttachmentId::new(456),
            name: "test-file.pdf".to_string(),
            size: 1024,
            created_user: create_mock_user(),
            created: Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap(),
        };

        let json = serde_json::to_value(&attachment).unwrap();
        assert_eq!(json["id"], 456);
        assert_eq!(json["name"], "test-file.pdf");
        assert_eq!(json["size"], 1024);
        assert_eq!(json["createdUser"]["id"], 123);
    }

    #[test]
    fn test_wiki_attachment_deserialization() {
        let json = r#"{
            "id": 789,
            "name": "document.docx",
            "size": 2048,
            "createdUser": {
                "id": 456,
                "userId": "testuser2",
                "name": "Test User 2",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "test2@example.com",
                "lastLoginTime": "2024-01-02T12:00:00Z"
            },
            "created": "2024-01-02T10:00:00Z"
        }"#;

        let attachment: WikiAttachment = serde_json::from_str(json).unwrap();
        assert_eq!(attachment.id.value(), 789);
        assert_eq!(attachment.name, "document.docx");
        assert_eq!(attachment.size, 2048);
        assert_eq!(attachment.created_user.name, "Test User 2");
    }
}
