use crate::models::{WikiAttachment, WikiTag};
use backlog_core::{
    Star, User,
    identifier::{ProjectId, WikiId},
};
use backlog_file::models::SharedFile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents detailed information about a wiki page, including content, attachments, and stars.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikiDetail {
    /// The unique identifier for the wiki page.
    pub id: WikiId,
    /// The project ID this wiki page belongs to.
    pub project_id: ProjectId,
    /// The name/title of the wiki page.
    pub name: String,
    /// The content of the wiki page (usually in Markdown format).
    pub content: String,
    /// Tags associated with this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WikiTag>,
    /// File attachments associated with this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<WikiAttachment>,
    /// Shared files linked to this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_files: Vec<SharedFile>,
    /// Stars given to this wiki page.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stars: Vec<Star>,
    /// The user who created this wiki page.
    pub created_user: User,
    /// The timestamp when this wiki page was created.
    pub created: DateTime<Utc>,
    /// The user who last updated this wiki page.
    pub updated_user: User,
    /// The timestamp when this wiki page was last updated.
    pub updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::{
        Language, Role,
        identifier::{
            Identifier, ProjectId, SharedFileId, StarId, UserId, WikiAttachmentId, WikiId,
            WikiTagId,
        },
    };
    use backlog_file::models::{FileContent, SharedFile};
    use chrono::TimeZone;

    fn create_mock_user(id: u32, name: &str) -> User {
        User {
            id: UserId::new(id),
            user_id: Some(name.to_lowercase()),
            name: name.to_string(),
            role_type: Role::User,
            lang: Some(Language::English),
            mail_address: format!("{}@example.com", name.to_lowercase()),
            last_login_time: Some(Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap()),
        }
    }

    fn create_mock_wiki_detail() -> WikiDetail {
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();
        let updated_time = Utc.with_ymd_and_hms(2024, 1, 2, 15, 30, 0).unwrap();

        WikiDetail {
            id: WikiId::new(123),
            project_id: ProjectId::new(456),
            name: "API Documentation".to_string(),
            content: "# API Documentation\n\nThis page contains API documentation...".to_string(),
            tags: vec![WikiTag {
                id: WikiTagId::new(1),
                name: "documentation".to_string(),
            }],
            attachments: vec![WikiAttachment {
                id: WikiAttachmentId::new(789),
                name: "api-spec.pdf".to_string(),
                size: 2048,
                created_user: create_mock_user(1, "John"),
                created: created_time,
            }],
            shared_files: vec![SharedFile {
                id: SharedFileId::new(101112),
                project_id: ProjectId::new(456),
                dir: "/docs".to_string(),
                name: "shared-diagram.png".to_string(),
                created_user: create_mock_user(2, "Alice"),
                created: created_time,
                updated_user: None,
                updated: None,
                content: FileContent::File { size: 4096 },
            }],
            stars: vec![Star {
                id: StarId::new(131415),
                comment: Some("Very helpful documentation!".to_string()),
                url: "https://example.backlog.jp/view/PROJ-123".to_string(),
                presenter: create_mock_user(3, "Bob"),
                created: created_time,
            }],
            created_user: create_mock_user(1, "John"),
            created: created_time,
            updated_user: create_mock_user(2, "Alice"),
            updated: updated_time,
        }
    }

    #[test]
    fn test_wiki_detail_serialization() {
        let wiki_detail = create_mock_wiki_detail();
        let json = serde_json::to_value(&wiki_detail).unwrap();

        assert_eq!(json["id"], 123);
        assert_eq!(json["projectId"], 456);
        assert_eq!(json["name"], "API Documentation");
        assert!(
            json["content"]
                .as_str()
                .unwrap()
                .contains("API Documentation")
        );
        assert_eq!(json["tags"].as_array().unwrap().len(), 1);
        assert_eq!(json["attachments"].as_array().unwrap().len(), 1);
        assert_eq!(json["sharedFiles"].as_array().unwrap().len(), 1);
        assert_eq!(json["stars"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_wiki_detail_basic_structure() {
        let wiki_detail = create_mock_wiki_detail();
        assert_eq!(wiki_detail.id.value(), 123);
        assert_eq!(wiki_detail.project_id.value(), 456);
        assert_eq!(wiki_detail.name, "API Documentation");
        assert!(wiki_detail.content.contains("API Documentation"));
        assert_eq!(wiki_detail.tags.len(), 1);
        assert_eq!(wiki_detail.attachments.len(), 1);
        assert_eq!(wiki_detail.shared_files.len(), 1);
        assert_eq!(wiki_detail.stars.len(), 1);
    }

    #[test]
    fn test_wiki_detail_with_minimal_data() {
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();

        let wiki_detail = WikiDetail {
            id: WikiId::new(999),
            project_id: ProjectId::new(777),
            name: "Minimal Wiki".to_string(),
            content: "Simple content".to_string(),
            tags: vec![],
            attachments: vec![],
            shared_files: vec![],
            stars: vec![],
            created_user: create_mock_user(1, "Creator"),
            created: created_time,
            updated_user: create_mock_user(1, "Creator"),
            updated: created_time,
        };

        // Test that empty collections are properly serialized/deserialized
        let json = serde_json::to_string(&wiki_detail).unwrap();
        let deserialized: WikiDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(wiki_detail, deserialized);
    }
}
