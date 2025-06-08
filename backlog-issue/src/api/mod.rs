use backlog_api_core::Result;
use backlog_core::{Identifier, IssueIdOrKey, IssueKey};
use client::{Client, DownloadedFile};

use crate::{
    models::{attachment::Attachment, comment::Comment, issue::Issue},
    requests::{
        AddIssueParams, CountIssueParams, GetIssueListParams, UpdateIssueParams,
        get_comment_list::GetCommentListParams,
    },
    responses::CountIssueResponse,
};

pub struct IssueApi(Client);

impl IssueApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_issue(
        &self,
        issue_id_or_key: impl Into<IssueIdOrKey>,
    ) -> Result<GetIssueResponse> {
        let issue_id_or_key_str: String = issue_id_or_key.into().into();
        self.0
            .get(&format!("/api/v2/issues/{}", issue_id_or_key_str))
            .await
    }

    pub async fn get_issue_list(&self, params: GetIssueListParams) -> Result<GetIssueListResponse> {
        let params_vec: Vec<(String, String)> = params.into();
        self.0.get_with_params("/api/v2/issues", &params_vec).await
    }

    pub async fn count_issue(&self, params: CountIssueParams) -> Result<CountIssueResponse> {
        let params: Vec<(String, String)> = params.into();
        self.0
            .get_with_params("/api/v2/issues/count", &params)
            .await
    }

    #[cfg(feature = "writable")]
    pub async fn add_issue(&self, params: AddIssueParams) -> Result<AddIssueResponse> {
        self.0.post("/api/v2/issues", &params).await
    }

    #[cfg(feature = "writable")]
    pub async fn delete_issue(
        &self,
        issue_key: impl Into<IssueKey>,
    ) -> Result<DeleteIssueResponse> {
        self.0
            .delete(&format!("/api/v2/issues/{}", issue_key.into()))
            .await
    }

    #[cfg(feature = "writable")]
    pub async fn update_issue(
        &self,
        issue_id_or_key: impl Into<IssueIdOrKey>,
        params: &UpdateIssueParams,
    ) -> Result<UpdateIssueResponse> {
        let issue_id_or_key_str: String = issue_id_or_key.into().into();
        self.0
            .patch(&format!("/api/v2/issues/{}", issue_id_or_key_str), params)
            .await
    }

    pub async fn get_comment_list(
        &self,
        issue_id_or_key: impl Into<IssueIdOrKey>,
        params: Option<GetCommentListParams>,
    ) -> Result<GetCommentListResponse> {
        let issue_key_str = issue_id_or_key.into().to_string();
        let path = format!("/api/v2/issues/{}/comments", issue_key_str);
        let query_params = params.map_or_else(Vec::new, |p| p.to_query_params());
        self.0.get_with_params(&path, &query_params).await
    }

    pub async fn get_attachment_list(
        &self,
        issue_id_or_key: impl Into<IssueIdOrKey>,
    ) -> Result<GetAttachmentListResponse> {
        let issue_key_str = issue_id_or_key.into().to_string();
        let path = format!("/api/v2/issues/{}/attachments", issue_key_str);
        self.0.get(&path).await
    }

    pub async fn get_attachment_file(
        &self,
        issue_id_or_key: impl Into<IssueIdOrKey>,
        attachment_id: backlog_core::identifier::AttachmentId,
    ) -> backlog_api_core::Result<DownloadedFile> {
        let issue_id_or_key_str = issue_id_or_key.into().to_string();
        let attachment_id_val = attachment_id.value();
        let path = format!(
            "/api/v2/issues/{}/attachments/{}",
            issue_id_or_key_str, attachment_id_val
        );
        self.0.download_file_raw(&path).await
    }
}

type GetIssueResponse = Issue;
type AddIssueResponse = Issue;
type DeleteIssueResponse = Issue;
type UpdateIssueResponse = Issue;
type GetIssueListResponse = Vec<Issue>;
type GetCommentListResponse = Vec<Comment>;
type GetAttachmentListResponse = Vec<Attachment>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{attachment::Attachment, comment::Comment},
        requests::{
            GetIssueListParamsBuilder,
            get_comment_list::{CommentOrder, GetCommentListParamsBuilder},
        },
    };
    use backlog_api_core::bytes::Bytes;
    use backlog_core::{
        IssueKey, User,
        identifier::{AttachmentId, CommentId, IssueId, ProjectId, UserId},
    };
    use chrono::{TimeZone, Utc};
    use client::test_utils::setup_client;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_mock_user(id: u32, name: &str) -> User {
        User {
            id: UserId::new(id),
            user_id: Some(name.to_string()),
            name: name.to_string(),
            role_type: backlog_core::Role::User,
            lang: Some(backlog_core::Language::Japanese),
            mail_address: format!("{}@example.com", name),
            last_login_time: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        }
    }

    // Note: create_mock_comment is not used by the new attachment tests, but kept for existing tests.
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

    #[tokio::test]
    async fn test_get_issue_list_empty_params_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);

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
                "status": {"id": 1, "projectId": 1, "name": "Open", "color": "#ff0000", "displayOrder": 1} // Full status
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
                "status": {"id": 2, "projectId": 1, "name": "In Progress", "color": "#0000ff", "displayOrder": 2} // Full status
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
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);
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
                "status": {"id": 1, "projectId": 123, "name": "Open", "color": "#ff0000", "displayOrder": 1} // Full status
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
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);

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
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-1";

        let expected_comments = vec![
            create_mock_comment(1, "First comment", 101, "alice"),
            create_mock_comment(2, "Second comment", 102, "bob"),
        ];

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/comments", issue_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comments))
            .mount(&mock_server)
            .await;
        let result = issue_api
            .get_comment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()), None)
            .await;
        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 2);
    }

    #[tokio::test]
    async fn test_get_comment_list_with_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);
        let issue_id = 123;

        let expected_comments = vec![create_mock_comment(5, "Comment with params", 103, "carol")];

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/comments", issue_id)))
            .and(query_param("count", "1"))
            .and(query_param("order", "asc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comments))
            .mount(&mock_server)
            .await;
        let params = GetCommentListParamsBuilder::default()
            .count(1u8)
            .order(CommentOrder::Asc)
            .build()
            .unwrap();
        let result = issue_api
            .get_comment_list(
                IssueIdOrKey::Id(IssueId::new(issue_id as u32)),
                Some(params),
            )
            .await;
        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 1);
    }

    #[tokio::test]
    async fn test_get_comment_list_empty_result() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-204";

        let expected_comments: Vec<Comment> = vec![];

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/comments", issue_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comments))
            .mount(&mock_server)
            .await;
        let result = issue_api
            .get_comment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()), None)
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_comment_list_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-404";

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/comments", issue_key)))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;
        let result = issue_api
            .get_comment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()), None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_comment_list_with_all_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-500";

        let expected_comments = vec![create_mock_comment(
            10,
            "Comment for all params test",
            104,
            "dave",
        )];

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/comments", issue_key)))
            .and(query_param("minId", "5"))
            .and(query_param("maxId", "15"))
            .and(query_param("count", "1"))
            .and(query_param("order", "desc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_comments))
            .mount(&mock_server)
            .await;
        let params = GetCommentListParamsBuilder::default()
            .min_id(5u64)
            .max_id(15u64)
            .count(1u8)
            .order(CommentOrder::Desc)
            .build()
            .unwrap();
        let result = issue_api
            .get_comment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()), Some(params))
            .await;
        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 1);
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
            created: chrono::DateTime::parse_from_rfc3339(created_str) // Use RFC3339 specific parser
                .unwrap()
                .with_timezone(&Utc),
        }
    }

    #[tokio::test]
    async fn test_get_attachment_list_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-1"; // Fixed issue key

        let expected_attachments = vec![
            create_mock_attachment(1, "file1.txt", 1024, 101, "alice", "2024-01-01T10:00:00Z"),
            create_mock_attachment(2, "image.png", 20480, 102, "bob", "2024-01-02T11:00:00Z"),
        ];

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/attachments", issue_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
            .mount(&mock_server)
            .await;

        let result = issue_api
            .get_attachment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .await;
        assert!(result.is_ok());
        let attachments = result.unwrap();
        assert_eq!(attachments.len(), 2);
        assert_eq!(attachments[0].name, "file1.txt");
        assert_eq!(attachments[1].size, 20480);
    }

    #[tokio::test]
    async fn test_get_attachment_list_empty() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-2"; // Fixed issue key

        let expected_attachments: Vec<Attachment> = vec![];

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/attachments", issue_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
            .mount(&mock_server)
            .await;

        let result = issue_api
            .get_attachment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_attachment_list_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-404"; // Fixed issue key

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/attachments", issue_key)))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = issue_api
            .get_attachment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_attachment_file_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key_str = "TESTPROJ-1";
        let attachment_id_val: u32 = 101;
        let issue_id_or_key: IssueIdOrKey = issue_key_str.parse::<IssueKey>().unwrap().into();
        let attachment_id = AttachmentId::new(attachment_id_val);

        let expected_body_bytes = Bytes::from_static(b"sample file content");
        let expected_filename = "test_attachment.dat";
        let expected_content_type = "application/octet-stream";

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/issues/{}/attachments/{}",
                issue_key_str, attachment_id_val
            )))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(expected_body_bytes.clone())
                    .insert_header("Content-Type", expected_content_type) // Use string literal
                    .insert_header(
                        "Content-Disposition", // Already string literal, ensure it stays
                        format!("attachment; filename=\"{}\"", expected_filename),
                    ),
            )
            .mount(&mock_server)
            .await;

        let result = issue_api
            .get_attachment_file(issue_id_or_key, attachment_id)
            .await;

        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, expected_filename);
        assert_eq!(downloaded_file.content_type, expected_content_type);
        assert_eq!(downloaded_file.bytes, expected_body_bytes);
    }

    #[tokio::test]
    async fn test_get_attachment_file_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let issue_key_str = "TESTPROJ-1";
        let attachment_id_val: u32 = 999; // Non-existent
        let issue_id_or_key: IssueIdOrKey = issue_key_str.parse::<IssueKey>().unwrap().into();
        let attachment_id = AttachmentId::new(attachment_id_val);

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/issues/{}/attachments/{}",
                issue_key_str, attachment_id_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = issue_api
            .get_attachment_file(issue_id_or_key, attachment_id)
            .await;
        assert!(result.is_err());
        // Optionally, check for specific error type if ApiError exposes it well
        // e.g., matches!(result.unwrap_err(), backlog_api_core::Error::HttpStatus { status: reqwest::StatusCode::NOT_FOUND, .. })
    }
}
