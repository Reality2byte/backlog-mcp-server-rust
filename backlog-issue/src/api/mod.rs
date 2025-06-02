use backlog_api_core::Result;
use backlog_core::{IssueIdOrKey, IssueKey, ProjectIdOrKey};
use client::Client;

use crate::{
    models::{comment::Comment, issue::Issue, issue::Milestone}, // Adjusted for Comment
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

    pub async fn get_version_milestone_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<GetVersionMilestoneListResponse> {
        let project_id_or_key_val = project_id_or_key.into();
        let project_id_or_key_str: String = project_id_or_key_val.to_string();
        self.0
            .get(&format!(
                "/api/v2/projects/{}/versions",
                project_id_or_key_str
            ))
            .await
    }

    /// Gets a list of comments for a specified issue.
    ///
    /// Corresponds to the [Get Comment List API](https://developer.nulab.com/ja/docs/backlog/api/2/get-comment-list/).
    ///
    /// # Arguments
    ///
    /// * `issue_id_or_key` - The ID or key of the issue for which to retrieve comments.
    /// * `params` - Optional parameters to filter and paginate the comment list.
    ///   See [`GetCommentListParams`](crate::requests::get_comment_list::GetCommentListParams) for details.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use backlog_api_client::BacklogApiClient;
    /// # use backlog_issue::requests::get_comment_list::{GetCommentListParamsBuilder, CommentOrder};
    /// # use backlog_core::IssueIdOrKey;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = BacklogApiClient::new("YOUR_SPACE_KEY", "YOUR_API_KEY", ".backlog.jp")?;
    /// let issue_key = IssueIdOrKey::key("PROJECT_KEY-123");
    /// let params = GetCommentListParamsBuilder::default()
    ///     .count(10u8)
    ///     .order(CommentOrder::Asc)
    ///     .build()?;
    ///
    /// let comments = client.issue().get_comment_list(issue_key, Some(params)).await?;
    ///
    /// for comment in comments {
    ///     println!("Comment ID: {}, Content: {:?}", comment.id, comment.content.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_comment_list(
        // Removed 'a lifetime from here
        &self,
        issue_id_or_key: impl Into<IssueIdOrKey>, // Removed 'a lifetime from here
        params: Option<GetCommentListParams>,
    ) -> Result<GetCommentListResponse> {
        let issue_key_str = issue_id_or_key.into().to_string();
        let path = format!("/api/v2/issues/{}/comments", issue_key_str);
        let query_params = params.map_or_else(Vec::new, |p| p.to_query_params());
        self.0.get_with_params(&path, &query_params).await
    }
}

type GetIssueResponse = Issue;
type AddIssueResponse = Issue;
type DeleteIssueResponse = Issue;
type UpdateIssueResponse = Issue;
type GetIssueListResponse = Vec<Issue>;
type GetCommentListResponse = Vec<Comment>; // Added this line
type GetVersionMilestoneListResponse = Vec<Milestone>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{
            comment::Comment, // Removed ChangeLogEntry, Notification, Star as they are not directly used in tests
            issue::Milestone,
        },
        requests::{
            GetIssueListParamsBuilder,
            get_comment_list::{CommentOrder, GetCommentListParamsBuilder}, // Added for comment tests
        },
    };
    use backlog_core::{
        User,                                                  // Added User
        identifier::{IssueId, MilestoneId, ProjectId, UserId}, // Added IssueId
    };
    use chrono::{TimeZone, Utc}; // Added Utc
    use client::Client;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_client(mock_server: &MockServer) -> Client {
        Client::new(&mock_server.uri()).expect("Failed to create client")
    }

    fn create_mock_user(id: u32, name: &str) -> User {
        // Changed id to u32
        User {
            id: UserId::new(id), // Now matches UserId::new requirement
            user_id: Some(name.to_string()),
            name: name.to_string(),
            role_type: backlog_core::Role::Developer, // Corrected Role
            lang: Some(backlog_core::Language::Japanese), // Corrected Language
            mail_address: format!("{}@example.com", name),
            last_login_time: "2024-01-01T00:00:00Z".to_string(), // Mocked last_login_time
        }
    }

    fn create_mock_comment(id: u64, content: &str, user_id: u32, user_name: &str) -> Comment {
        // Changed user_id to u32
        let created_time = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        Comment {
            id,
            content: Some(content.to_string()),
            change_log: vec![],
            created_user: create_mock_user(user_id, user_name), // Pass u32
            created: created_time,
            updated: created_time,
            stars: vec![],
            notifications: vec![],
        }
    }

    #[tokio::test]
    async fn test_get_issue_list_empty_params_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
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
                "status": {"id": 1, "name": "Open"}
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
                "status": {"id": 2, "name": "In Progress"}
            })).unwrap(),
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/issues"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issues))
            .mount(&mock_server)
            .await;

        let params = GetIssueListParamsBuilder::default().build().unwrap();

        // Act
        let result = issue_api.get_issue_list(params).await;

        // Assert
        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].id, expected_issues[0].id);
        assert_eq!(issues[1].summary, expected_issues[1].summary);
    }

    #[tokio::test]
    async fn test_get_issue_list_with_project_id_param() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
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
                "status": {"id": 1, "name": "Open"}
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

        // Act
        let result = issue_api.get_issue_list(params).await;

        // Assert
        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].project_id, project_id);
    }

    #[tokio::test]
    async fn test_get_issue_list_server_error() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/issues"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = GetIssueListParamsBuilder::default().build().unwrap();

        // Act
        let result = issue_api.get_issue_list(params).await;

        // Assert
        assert!(result.is_err());
        // Further error type inspection could be done here
    }

    #[tokio::test]
    async fn test_get_version_milestone_list_success() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let project_id_or_key_str = "TEST_PROJECT";
        let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();
        let project_id_numeric = ProjectId::new(1); // Assuming project ID 1 for mock

        let expected_versions: Vec<Milestone> = vec![
            Milestone {
                id: MilestoneId::new(1),
                project_id: project_id_numeric,
                name: "Version 1.0".to_string(),
                description: Some("Initial release".to_string()),
                start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
                release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
                archived: false,
                display_order: Some(1),
            },
            Milestone {
                id: MilestoneId::new(2),
                project_id: project_id_numeric,
                name: "Version 1.1".to_string(),
                description: None,
                start_date: None,
                release_due_date: None,
                archived: true,
                display_order: Some(2),
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/versions",
                project_id_or_key.clone()
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_versions))
            .mount(&mock_server)
            .await;

        // Act
        let result = issue_api
            .get_version_milestone_list(project_id_or_key.clone())
            .await;

        // Assert
        assert!(result.is_ok());
        let versions = result.unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].name, "Version 1.0");
        assert!(versions[1].archived);
        assert_eq!(versions[0].display_order, Some(1));
    }

    #[tokio::test]
    async fn test_get_version_milestone_list_error() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let project_id_or_key_str = "TEST_PROJECT_ERROR";
        let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/versions",
                project_id_or_key.clone()
            )))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        // Act
        let result = issue_api
            .get_version_milestone_list(project_id_or_key.clone())
            .await;

        // Assert
        assert!(result.is_err());
    }

    // --- Tests for get_comment_list ---

    #[tokio::test]
    async fn test_get_comment_list_success_no_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-1"; // Made key more typical

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
        assert_eq!(comments[0].id, 1);
        assert_eq!(comments[1].content.as_deref(), Some("Second comment"));
    }

    #[tokio::test]
    async fn test_get_comment_list_with_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_id = 123; // Using ID this time

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
            ) // Corrected path to IssueId
            .await;

        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].id, 5);
    }

    #[tokio::test]
    async fn test_get_comment_list_empty_result() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-204"; // Corrected key format

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
        let comments = result.unwrap();
        assert!(comments.is_empty());
    }

    #[tokio::test]
    async fn test_get_comment_list_issue_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-404"; // Corrected key format

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/issues/{}/comments", issue_key)))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = issue_api
            .get_comment_list(IssueIdOrKey::Key(issue_key.parse().unwrap()), None)
            .await;

        assert!(result.is_err());
        // Optionally, check the specific error type if your client maps 404 to a specific variant
    }

    #[tokio::test]
    async fn test_get_comment_list_with_all_params() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let issue_key = "TESTKEY-500"; // Corrected key format

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
        assert_eq!(comments[0].id, 10);
    }
}
