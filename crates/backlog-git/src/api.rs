#[cfg(feature = "writable")]
use crate::requests::add_pull_request_comment::AddPullRequestCommentParams;
use crate::{
    models::{PullRequest, PullRequestAttachment, PullRequestComment, Repository},
    requests::get_pull_request_comment_list::GetPullRequestCommentListParams,
};
use backlog_api_core::Result;
use backlog_core::{
    ProjectIdOrKey, RepositoryIdOrName,
    identifier::{Identifier, PullRequestAttachmentId, PullRequestNumber},
};
use client::{Client, DownloadedFile};

/// Provides access to the Git and Pull Request related API functions.
#[derive(Debug, Clone)]
pub struct GitApi {
    client: Client,
}

impl GitApi {
    /// Creates a new GitApi.
    ///
    /// This is typically called by `BacklogApiClient::git()`.
    ///
    /// # Arguments
    ///
    /// * `client` - An instance of the generic `client::Client`.
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Fetches the list of Git repositories for a given project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories`.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    pub async fn get_repository_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<Vec<Repository>> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories",
            project_id_or_key.into()
        );
        self.client.get(&path).await
    }

    /// Fetches a single Git repository by its ID or name.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName`.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    /// * `repo_id_or_name` - The ID (as a string) or name of the repository.
    pub async fn get_repository(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
    ) -> Result<Repository> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}",
            project_id_or_key.into(),
            repo_id_or_name.into()
        );
        self.client.get(&path).await
    }

    /// Fetches the list of Pull Requests for a given repository.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests`.
    /// TODO: Add support for query parameters (e.g., status, assignee) via a params struct.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    /// * `repo_id_or_name` - The ID (as a string) or name of the repository.
    pub async fn get_pull_request_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
    ) -> Result<Vec<PullRequest>> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests",
            project_id_or_key.into(),
            repo_id_or_name.into()
        );
        self.client.get(&path).await
    }

    /// Fetches a single Pull Request by its number.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number`.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    /// * `repo_id_or_name` - The ID (as a string) or name of the repository.
    /// * `pr_number` - The pull request number.
    pub async fn get_pull_request(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
    ) -> Result<PullRequest> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
            project_id_or_key.into(),
            repo_id_or_name.into(),
            pr_number.value()
        );
        self.client.get(&path).await
    }

    // TODO:
    // - (Consider request parameter structs like ListPullRequestsParams for query options for list_pull_requests)
    // - Consider creating a RepositoryIdOrName type in backlog-core for repo_id_or_name.
    // - Implement functions for PR comments and attachments if needed.
    // - Implement functions for creating/updating PRs if writable features are desired.

    /// Fetches the list of attachments for a specific pull request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/repositories/:repoIdOrName/pullRequests/:number/attachments`.
    pub async fn get_pull_request_attachment_list(
        &self,
        project_id_or_key: &ProjectIdOrKey, // Keeping as reference based on existing code
        repo_id_or_name: &RepositoryIdOrName, // Keeping as reference
        pr_number: PullRequestNumber,
    ) -> backlog_api_core::Result<Vec<PullRequestAttachment>> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments",
            project_id_or_key, // Display trait を利用
            repo_id_or_name,   // Display trait を利用
            pr_number.value()
        );
        self.client.get(&path).await // クエリパラメータはなし
    }

    /// Downloads the content of a specific pull request attachment.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId`.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    /// * `repo_id_or_name` - The ID (as a string) or name of the repository.
    /// * `pr_number` - The pull request number.
    /// * `attachment_id` - The ID of the attachment to download.
    pub async fn download_pull_request_attachment(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
        attachment_id: PullRequestAttachmentId,
    ) -> Result<DownloadedFile> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
            project_id_or_key.into(),
            repo_id_or_name.into(),
            pr_number.value(),
            attachment_id.value(),
        );
        self.client.download_file_raw(&path).await
    }

    /// Fetches the list of comments for a specific pull request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments`.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    /// * `repo_id_or_name` - The ID (as a string) or name of the repository.
    /// * `pr_number` - The pull request number.
    /// * `params` - Optional query parameters for filtering and pagination.
    pub async fn get_pull_request_comment_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
        params: GetPullRequestCommentListParams,
    ) -> Result<Vec<PullRequestComment>> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
            project_id_or_key.into(),
            repo_id_or_name.into(),
            pr_number.value()
        );
        self.client.get_with_params(&path, &params).await
    }

    /// Adds a comment to a specific pull request.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments`.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    /// * `repo_id_or_name` - The ID (as a string) or name of the repository.
    /// * `pr_number` - The pull request number.
    /// * `params` - Parameters for the comment including content and optional user notifications.
    #[cfg(feature = "writable")]
    pub async fn add_pull_request_comment(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        repo_id_or_name: impl Into<RepositoryIdOrName>,
        pr_number: PullRequestNumber,
        params: &AddPullRequestCommentParams,
    ) -> Result<PullRequestComment> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
            project_id_or_key.into(),
            repo_id_or_name.into(),
            pr_number.value()
        );
        let params_vec: Vec<(String, String)> = params.into();
        self.client.post(&path, &params_vec).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // backlog_api_core::bytes is already in scope from the top-level import if we change it there.
    // No, the top level import is `backlog_api_core::bytes`, so here we'd use `bytes::Bytes`.
    // Or, import `backlog_api_core::bytes::Bytes` specifically for the test module if preferred.
    // Let's rely on the top-level `bytes` module being available.
    use crate::models::PrCommentOrder;
    #[cfg(feature = "writable")]
    use crate::requests::add_pull_request_comment::{
        AddPullRequestCommentParams, AddPullRequestCommentParamsBuilder,
    };
    use crate::requests::get_pull_request_comment_list::GetPullRequestCommentListParamsBuilder;
    use backlog_api_core::bytes::Bytes;
    use backlog_core::identifier::{
        Identifier, PullRequestAttachmentId, PullRequestCommentId, PullRequestNumber, UserId,
    };
    use client::test_utils::setup_client;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_pull_request_attachment_list_success_multiple_attachments() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 123;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = vec![
            PullRequestAttachment {
                id: PullRequestAttachmentId::new(101),
                name: "image.png".to_string(),
                size: 12345,
            },
            PullRequestAttachment {
                id: PullRequestAttachmentId::new(102),
                name: "document.pdf".to_string(),
                size: 67890,
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key = project_key.parse().unwrap();
        let repo_id_or_name = repo_name.parse().unwrap();

        let result = git_api
            .get_pull_request_attachment_list(&project_id_or_key, &repo_id_or_name, pr_number)
            .await;

        assert!(result.is_ok());
        let attachments = result.unwrap();
        assert_eq!(attachments.len(), 2);
        assert_eq!(attachments[0].id.value(), 101);
        assert_eq!(attachments[0].name, "image.png");
        assert_eq!(attachments[1].id.value(), 102);
        assert_eq!(attachments[1].name, "document.pdf");
    }

    #[tokio::test]
    async fn test_get_pull_request_attachment_list_success_no_attachments() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 124;
        let pr_number = PullRequestNumber::new(pr_number_val);
        let mock_response: Vec<PullRequestAttachment> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key = project_key.parse().unwrap();
        let repo_id_or_name = repo_name.parse().unwrap();

        let result = git_api
            .get_pull_request_attachment_list(&project_id_or_key, &repo_id_or_name, pr_number)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_pull_request_attachment_list_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "NONEXISTENT";
        let repo_name = "norepo";
        let pr_number_val = 1;
        let pr_number = PullRequestNumber::new(pr_number_val);

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key = project_key.parse().unwrap();
        let repo_id_or_name = repo_name.parse().unwrap();

        let result = git_api
            .get_pull_request_attachment_list(&project_id_or_key, &repo_id_or_name, pr_number)
            .await;

        assert!(result.is_err());
        // Further assertions can be made on the error type if needed
        // e.g., matches!(result.unwrap_err(), backlog_api_core::Error::HttpStatus { .. })
    }

    #[tokio::test]
    async fn test_download_pull_request_attachment_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 125;
        let pr_number = PullRequestNumber::new(pr_number_val);
        let attachment_id_val = 201;
        let attachment_content = "This is a test file content.";

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
                project_key, repo_name, pr_number_val, attachment_id_val
            )))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(attachment_content)
                    .insert_header("Content-Type", "application/octet-stream")
                    .insert_header(
                        "Content-Disposition",
                        "attachment; filename=\"test_file.txt\"",
                    ),
            )
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let attachment_id = PullRequestAttachmentId::new(attachment_id_val);

        let result = git_api
            .download_pull_request_attachment(
                project_id_or_key,
                repo_id_or_name,
                pr_number,
                attachment_id,
            )
            .await;

        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, "test_file.txt");
        assert_eq!(downloaded_file.content_type, "application/octet-stream");
        assert_eq!(downloaded_file.bytes, Bytes::from(attachment_content));
    }

    #[tokio::test]
    async fn test_download_pull_request_attachment_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 126;
        let pr_number = PullRequestNumber::new(pr_number_val);
        let attachment_id_val = 202;

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
                project_key, repo_name, pr_number_val, attachment_id_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let attachment_id = PullRequestAttachmentId::new(attachment_id_val);

        let result = git_api
            .download_pull_request_attachment(
                project_id_or_key,
                repo_id_or_name,
                pr_number,
                attachment_id,
            )
            .await;

        assert!(result.is_err());
        // Example: Check for specific error kind if ApiError is structured enough
        // match result.unwrap_err() {
        //     backlog_api_core::Error::HttpStatus { status, .. } => assert_eq!(status, reqwest::StatusCode::NOT_FOUND),
        //     _ => panic!("Expected HttpStatus error"),
        // }
    }

    #[tokio::test]
    async fn test_get_pull_request_comment_list_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 127;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!([
            {
                "id": 35,
                "content": "from api",
                "changeLog": [],
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "admin",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "eguchi@nulab.example"
                },
                "created": "2015-05-14T01:53:38Z",
                "updated": "2015-05-14T01:53:38Z",
                "stars": [],
                "notifications": []
            }
        ]);

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let result = git_api
            .get_pull_request_comment_list(
                project_id_or_key,
                repo_id_or_name,
                pr_number,
                GetPullRequestCommentListParams::default(),
            )
            .await;

        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].id, PullRequestCommentId(35));
        assert_eq!(comments[0].content, "from api");
        assert_eq!(comments[0].created_user.id, UserId(1));
    }

    #[tokio::test]
    async fn test_get_pull_request_comment_list_with_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 128;
        let pr_number = PullRequestNumber::new(pr_number_val);

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
                project_key, repo_name, pr_number_val
            )))
            .and(wiremock::matchers::query_param("count", "1"))
            .and(wiremock::matchers::query_param("order", "asc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = GetPullRequestCommentListParamsBuilder::default()
            .count(Some(1))
            .order(Some(PrCommentOrder::Asc))
            .build()
            .unwrap();

        let result = git_api
            .get_pull_request_comment_list(project_id_or_key, repo_id_or_name, pr_number, params)
            .await;

        assert!(result.is_ok());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_comment_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 129;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!({
            "id": 36,
            "content": "This is a test comment",
            "changeLog": [],
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-01T12:00:00Z",
            "updated": "2024-01-01T12:00:00Z",
            "stars": [],
            "notifications": []
        });

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(201).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = AddPullRequestCommentParams::new("This is a test comment");

        let result = git_api
            .add_pull_request_comment(project_id_or_key, repo_id_or_name, pr_number, &params)
            .await;

        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id, PullRequestCommentId(36));
        assert_eq!(comment.content, "This is a test comment");
        assert_eq!(comment.created_user.id, UserId(1));
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_comment_with_notifications() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 130;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!({
            "id": 37,
            "content": "Comment with notifications",
            "changeLog": [],
            "createdUser": {
                "id": 2,
                "userId": "user",
                "name": "User",
                "roleType": 2,
                "lang": "ja",
                "mailAddress": "user@example.com"
            },
            "created": "2024-01-01T13:00:00Z",
            "updated": "2024-01-01T13:00:00Z",
            "stars": [],
            "notifications": []
        });

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(201).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = AddPullRequestCommentParamsBuilder::default()
            .content("Comment with notifications".to_string())
            .notified_user_ids(Some(vec![UserId::new(101), UserId::new(102)]))
            .build()
            .unwrap();

        let result = git_api
            .add_pull_request_comment(project_id_or_key, repo_id_or_name, pr_number, &params)
            .await;
        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id, PullRequestCommentId(37));
        assert_eq!(comment.content, "Comment with notifications");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_comment_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "NONEXISTENT";
        let repo_name = "norepo";
        let pr_number_val = 999;
        let pr_number = PullRequestNumber::new(pr_number_val);

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = AddPullRequestCommentParams::new("This comment should fail");

        let result = git_api
            .add_pull_request_comment(project_id_or_key, repo_id_or_name, pr_number, &params)
            .await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_comment_parameter_builder() {
        let params_with_all_fields = AddPullRequestCommentParamsBuilder::default()
            .content("Test content".to_string())
            .notified_user_ids(Some(vec![UserId::new(1), UserId::new(2), UserId::new(3)]))
            .build()
            .unwrap();

        assert_eq!(params_with_all_fields.content, "Test content");
        assert_eq!(
            params_with_all_fields.notified_user_ids,
            Some(vec![UserId::new(1), UserId::new(2), UserId::new(3)])
        );

        let params_minimal = AddPullRequestCommentParamsBuilder::default()
            .content("Minimal content".to_string())
            .build()
            .unwrap();

        assert_eq!(params_minimal.content, "Minimal content");
        assert_eq!(params_minimal.notified_user_ids, None);

        let params_from_new = AddPullRequestCommentParams::new("From new method");
        assert_eq!(params_from_new.content, "From new method");
        assert_eq!(params_from_new.notified_user_ids, None);
    }
}
