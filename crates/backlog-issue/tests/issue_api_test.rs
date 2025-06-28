mod common;
use backlog_core::{IssueKey, identifier::Identifier};
use common::*;

use backlog_issue::{
    CommentOrder, CountCommentParams, GetAttachmentListParams, GetCommentListParamsBuilder,
    GetCommentParams, GetIssueListParamsBuilder, GetParticipantListParams, GetSharedFileListParams,
};

fn create_mock_user(id: u32, name: &str) -> User {
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

fn create_mock_comment(id: u32, content: &str, user_id: u32, user_name: &str) -> Comment {
    let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    Comment {
        id: CommentId::new(id),
        content: Some(content.to_string()),
        change_log: vec![],
        created_user: create_mock_user(user_id, user_name),
        created: created_time,
        updated: created_time,
        stars: vec![],
        notifications: vec![],
    }
}

fn create_mock_attachment(
    id: u32,
    name: &str,
    size: u64,
    user_id: u32,
    user_name: &str,
    created_str: &str,
) -> Attachment {
    Attachment {
        id: AttachmentId::new(id),
        name: name.to_string(),
        size,
        created_user: create_mock_user(user_id, user_name),
        created: chrono::DateTime::parse_from_rfc3339(created_str)
            .unwrap()
            .with_timezone(&Utc),
    }
}

fn create_mock_shared_file(
    id: u32,
    dir: &str,
    name: &str,
    size: Option<u64>,
    user_id: u32,
    user_name: &str,
    created_str: &str,
) -> SharedFile {
    SharedFile {
        id: SharedFileId::new(id),
        dir: dir.to_string(),
        name: name.to_string(),
        created_user: create_mock_user(user_id, user_name),
        created: chrono::DateTime::parse_from_rfc3339(created_str)
            .unwrap()
            .with_timezone(&Utc),
        updated_user: None,
        updated: None,
        content: match size {
            Some(s) => FileContent::File { size: s },
            None => FileContent::Directory,
        },
    }
}

#[tokio::test]
async fn test_get_issue_list_empty_params_success() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    let expected_issues: Vec<Issue> = vec![
        serde_json::from_value(json!({
            "id": 1, "projectId": 1, "issueKey": "BLG-1", "keyId": 1, "summary": "Test Issue 1",
            "description": "This is a test issue (description)",
            "issueType": {"id": 1, "projectId":1, "name": "Bug", "color": "#ff0000", "displayOrder": 0},
            "priority": {"id": 2, "name": "High"},
            "category": [],
            "versions": [],
            "milestone": [],
            "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
            "created": "2024-03-14T06:35:39Z",
            "updated": "2024-04-13T06:35:39Z",
            "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ff0000", "displayOrder": 1}
        })).unwrap(),
        serde_json::from_value(json!({
            "id": 2, "projectId": 1, "issueKey": "BLG-2", "keyId": 2, "summary": "Test Issue 2",
            "description": "This is another test issue (description)",
            "issueType": {"id": 2, "projectId":1, "name": "Task", "color": "#00ff00", "displayOrder": 1},
            "priority": {"id": 3, "name": "Normal"},
            "category": [],
            "versions": [],
            "milestone": [],
            "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
            "created": "2024-03-14T06:35:39Z",
            "updated": "2024-04-13T06:35:39Z",
            "status": {"id": 2, "projectId": 1, "name": "In Progress", "color": "#0000ff", "displayOrder": 2}
        })).unwrap(),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/issues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issues))
        .mount(&mock_server)
        .await;

    let params = GetIssueListParamsBuilder::default().build().unwrap();
    let result = issue_api.get_issue_list(params).await;
    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 2);
    assert_eq!(issues[0].id, expected_issues[0].id);
    assert_eq!(issues[1].summary, expected_issues[1].summary);
}

#[tokio::test]
async fn test_get_issue_list_with_project_id_param() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let project_id = ProjectId::new(123);

    let expected_issues: Vec<Issue> = vec![
         serde_json::from_value(json!({
            "id": 3, "projectId": 123, "issueKey": "XYZ-3", "keyId": 3, "summary": "Filtered Issue",
            "issueType": {"id": 1, "projectId":123, "name": "Bug", "color": "#ff0000", "displayOrder": 0},
            "priority": {"id": 2, "name": "High"},
            "description": "This is another test issue (description)",
            "category": [],
            "versions": [],
            "milestone": [],
            "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
            "created": "2024-03-14T06:35:39Z",
            "updated": "2024-04-13T06:35:39Z",
            "status": {"id": 1, "projectId": 123, "name": "Open", "color": "#ff0000", "displayOrder": 1}
        })).unwrap(),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/issues"))
        .and(query_param("projectId[]", project_id.to_string()))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issues))
        .mount(&mock_server)
        .await;

    let params = GetIssueListParamsBuilder::default()
        .project_id(vec![project_id])
        .build()
        .unwrap();
    let result = issue_api.get_issue_list(params).await;
    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].project_id, project_id);
}

#[tokio::test]
async fn test_get_issue_list_server_error() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;

    Mock::given(method("GET"))
        .and(path("/api/v2/issues"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let params = GetIssueListParamsBuilder::default().build().unwrap();
    let result = issue_api.get_issue_list(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_comment_list_success_no_params() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_key = "TESTKEY-1";

    let expected_comments = vec![
        create_mock_comment(1, "First comment", 101, "alice"),
        create_mock_comment(2, "Second comment", 102, "bob"),
    ];

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/issues/{issue_key}/comments")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comments))
        .mount(&mock_server)
        .await;
    let params = GetCommentListParamsBuilder::default()
        .issue_id_or_key(IssueIdOrKey::Key(issue_key.parse().unwrap()))
        .build()
        .unwrap();
    let result = issue_api.get_comment_list(params).await;
    assert!(result.is_ok());
    let comments = result.unwrap();
    assert_eq!(comments.len(), 2);
}

#[tokio::test]
async fn test_get_comment_list_with_params() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_id = 123;

    let expected_comments = vec![create_mock_comment(5, "Comment with params", 103, "carol")];

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/issues/{issue_id}/comments")))
        .and(query_param("count", "1"))
        .and(query_param("order", "asc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comments))
        .mount(&mock_server)
        .await;
    let params = GetCommentListParamsBuilder::default()
        .issue_id_or_key(IssueIdOrKey::Id(IssueId::new(issue_id as u32)))
        .count(1u8)
        .order(CommentOrder::Asc)
        .build()
        .unwrap();
    let result = issue_api.get_comment_list(params).await;
    assert!(result.is_ok());
    let comments = result.unwrap();
    assert_eq!(comments.len(), 1);
}

#[tokio::test]
async fn test_get_attachment_list_success() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_key = "TESTKEY-1";

    let expected_attachments = vec![
        create_mock_attachment(1, "file1.txt", 1024, 101, "alice", "2024-01-01T10:00:00Z"),
        create_mock_attachment(2, "image.png", 20480, 102, "bob", "2024-01-02T11:00:00Z"),
    ];

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/issues/{issue_key}/attachments")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
        .mount(&mock_server)
        .await;

    let result = issue_api
        .get_attachment_list(GetAttachmentListParams::new(IssueIdOrKey::Key(
            issue_key.parse().unwrap(),
        )))
        .await;
    assert!(result.is_ok());
    let attachments = result.unwrap();
    assert_eq!(attachments.len(), 2);
    assert_eq!(attachments[0].name, "file1.txt");
    assert_eq!(attachments[1].size, 20480);
}

#[tokio::test]
async fn test_get_participant_list_success() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_key = "TESTKEY-1";

    let expected_participants = vec![
        create_mock_user(1, "admin"),
        create_mock_user(2, "alice"),
        create_mock_user(3, "bob"),
    ];

    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path(format!(
            "/api/v2/issues/{issue_key}/participants"
        )))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(&expected_participants))
        .mount(&mock_server)
        .await;

    let params = GetParticipantListParams::new(IssueKey::new("TESTKEY".parse().unwrap(), 1));
    let result = issue_api.get_participant_list(params).await;

    assert!(result.is_ok());
    let participants = result.unwrap();
    assert_eq!(participants.len(), 3);
    assert_eq!(participants[0].user_id, Some("admin".to_string()));
    assert_eq!(participants[1].user_id, Some("alice".to_string()));
    assert_eq!(participants[2].user_id, Some("bob".to_string()));
}

#[tokio::test]
async fn test_get_participant_list_empty() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_id = IssueId::new(12345);

    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path(format!(
            "/api/v2/issues/{}/participants",
            issue_id.value()
        )))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(Vec::<User>::new()))
        .mount(&mock_server)
        .await;

    let params = GetParticipantListParams::new(issue_id);
    let result = issue_api.get_participant_list(params).await;

    assert!(result.is_ok());
    let participants = result.unwrap();
    assert!(participants.is_empty());
}

#[tokio::test]
async fn test_get_participant_list_issue_not_found() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_key = "NONEXISTENT-1";

    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path(format!(
            "/api/v2/issues/{issue_key}/participants"
        )))
        .respond_with(
            wiremock::ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "No issue for the issueIdOrKey."}]
            })),
        )
        .mount(&mock_server)
        .await;

    let params = GetParticipantListParams::new(IssueKey::new("TESTKEY".parse().unwrap(), 1));
    let result = issue_api.get_participant_list(params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_shared_file_list_success() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_key = "TESTKEY-1";

    let expected_shared_files = vec![
        create_mock_shared_file(
            1,
            "/shared",
            "document.pdf",
            Some(2048),
            101,
            "alice",
            "2024-01-01T10:00:00Z",
        ),
        create_mock_shared_file(
            2,
            "/shared/images",
            "photo.jpg",
            Some(10240),
            102,
            "bob",
            "2024-01-02T11:00:00Z",
        ),
    ];

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/issues/{issue_key}/sharedFiles")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
        .mount(&mock_server)
        .await;

    let result = issue_api
        .get_shared_file_list(GetSharedFileListParams::new(IssueIdOrKey::Key(
            issue_key.parse().unwrap(),
        )))
        .await;
    assert!(result.is_ok());
    let shared_files = result.unwrap();
    assert_eq!(shared_files.len(), 2);
    assert_eq!(shared_files[0].name, "document.pdf");
    assert_eq!(shared_files[1].name, "photo.jpg");
}

#[tokio::test]
async fn test_count_comment_success() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_key = "TESTKEY-1";

    Mock::given(method("GET"))
        .and(path(format!("/api/v2/issues/{issue_key}/comments/count")))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "count": 5
        })))
        .mount(&mock_server)
        .await;

    let result = issue_api
        .count_comment(CountCommentParams::new(IssueIdOrKey::Key(
            issue_key.parse().unwrap(),
        )))
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.count, 5);
}

#[tokio::test]
async fn test_get_comment_success() {
    let mock_server = wiremock::MockServer::start().await;
    let issue_api = setup_issue_api(&mock_server).await;
    let issue_key = "TESTKEY-1";
    let comment_id = CommentId::new(123);

    let expected_comment = create_mock_comment(123, "This is a test comment", 101, "alice");

    Mock::given(method("GET"))
        .and(path(format!(
            "/api/v2/issues/{issue_key}/comments/{comment_id}"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comment))
        .mount(&mock_server)
        .await;

    let result = issue_api
        .get_comment(GetCommentParams::new(
            IssueIdOrKey::Key(issue_key.parse().unwrap()),
            comment_id,
        ))
        .await;

    assert!(result.is_ok());
    let comment = result.unwrap();
    assert_eq!(comment.id, CommentId::new(123));
    assert_eq!(comment.content, Some("This is a test comment".to_string()));
}
