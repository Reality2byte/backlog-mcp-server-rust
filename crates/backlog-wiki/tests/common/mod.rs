use backlog_wiki::WikiApi;
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
pub async fn setup_wiki_api(mock_server: &MockServer) -> WikiApi {
    let client = setup_client(mock_server).await;
    WikiApi::new(client)
}

/// Common imports for tests
pub use backlog_core::identifier::{
    Identifier, ProjectId, SharedFileId, StarId, UserId, WikiAttachmentId, WikiId, WikiTagId,
};
pub use backlog_core::{Language, Role, Star, User};
pub use backlog_file::models::{FileContent, SharedFile};
pub use backlog_wiki::models::{Wiki, WikiAttachment, WikiDetail, WikiHistory, WikiTag};
pub use chrono::{TimeZone, Utc};
pub use wiremock::{Mock, ResponseTemplate};

pub fn create_mock_user(id: u32, name: &str) -> User {
    User {
        id: UserId::new(id),
        user_id: Some(name.to_string()),
        name: name.to_string(),
        role_type: Role::User,
        lang: Some(Language::Japanese),
        mail_address: format!("{name}@example.com"),
        last_login_time: Some(
            chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        ),
    }
}

#[allow(dead_code)]
pub fn create_mock_wiki(
    id: u32,
    project_id: u32,
    name: &str,
    user_id: u32,
    user_name: &str,
) -> Wiki {
    let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    Wiki {
        id: WikiId::new(id),
        project_id: ProjectId::new(project_id),
        name: name.to_string(),
        tags: vec![WikiTag {
            id: WikiTagId::new(1),
            name: "proceedings".to_string(),
        }],
        created_user: create_mock_user(user_id, user_name),
        created: created_time,
        updated_user: create_mock_user(user_id, user_name),
        updated: created_time,
    }
}

pub fn create_mock_wiki_detail(id: u32, project_id: u32, name: &str) -> WikiDetail {
    let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 10, 0, 0).unwrap();
    let updated_time = Utc.with_ymd_and_hms(2024, 1, 2, 15, 30, 0).unwrap();

    WikiDetail {
        id: WikiId::new(id),
        project_id: ProjectId::new(project_id),
        name: name.to_string(),
        content: format!("# {name}\n\nThis is the content of {name}."),
        tags: vec![WikiTag {
            id: WikiTagId::new(1),
            name: "documentation".to_string(),
        }],
        attachments: vec![WikiAttachment {
            id: WikiAttachmentId::new(100),
            name: "attachment.pdf".to_string(),
            size: 1024,
            created_user: create_mock_user(1, "john"),
            created: created_time,
        }],
        shared_files: vec![SharedFile {
            id: SharedFileId::new(200),
            project_id: ProjectId::new(project_id),
            dir: "/docs".to_string(),
            name: "shared.png".to_string(),
            created_user: create_mock_user(2, "alice"),
            created: created_time,
            updated_user: None,
            updated: None,
            content: FileContent::File { size: 2048 },
        }],
        stars: vec![Star {
            id: StarId::new(300),
            comment: Some("Great documentation!".to_string()),
            url: format!("https://example.backlog.jp/view/PROJ-{id}"),
            presenter: create_mock_user(3, "bob"),
            created: created_time,
        }],
        created_user: create_mock_user(1, "john"),
        created: created_time,
        updated_user: create_mock_user(2, "alice"),
        updated: updated_time,
    }
}

#[allow(dead_code)]
pub fn create_mock_wiki_attachment(
    id: u32,
    name: &str,
    size: u64,
    user_id: u32,
    user_name: &str,
) -> WikiAttachment {
    let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    WikiAttachment {
        id: WikiAttachmentId::new(id),
        name: name.to_string(),
        size,
        created_user: create_mock_user(user_id, user_name),
        created: created_time,
    }
}

#[allow(dead_code)]
pub fn create_mock_wiki_history(
    page_id: u32,
    version: u32,
    name: &str,
    user_name: &str,
) -> WikiHistory {
    let created_time = Utc
        .with_ymd_and_hms(2024, 1, 1, 12, 0, 0)
        .unwrap()
        .checked_add_signed(chrono::Duration::hours(version as i64))
        .unwrap();

    WikiHistory {
        page_id: WikiId::new(page_id),
        version,
        name: name.to_string(),
        content: format!("Content for {name} version {version}"),
        created_user: create_mock_user(1, user_name),
        created: created_time,
    }
}

#[allow(dead_code)]
pub fn create_mock_shared_file(
    id: u32,
    project_id: u32,
    dir: &str,
    name: &str,
    size: u64,
    user_id: u32,
    user_name: &str,
) -> SharedFile {
    let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    SharedFile {
        id: SharedFileId::new(id),
        project_id: ProjectId::new(project_id),
        dir: dir.to_string(),
        name: name.to_string(),
        created_user: create_mock_user(user_id, user_name),
        created: created_time,
        updated_user: None,
        updated: None,
        content: FileContent::File { size },
    }
}
