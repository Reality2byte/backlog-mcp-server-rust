#[cfg(feature = "writable")]
use crate::requests::add_pull_request::AddPullRequestParams;
#[cfg(feature = "writable")]
use crate::requests::add_pull_request_comment::AddPullRequestCommentParams;
#[cfg(feature = "writable")]
use crate::requests::delete_pull_request_attachment::DeletePullRequestAttachmentParams;
#[cfg(feature = "writable")]
use crate::requests::update_pull_request::UpdatePullRequestParams;
#[cfg(feature = "writable")]
use crate::requests::update_pull_request_comment::UpdatePullRequestCommentParams;
use crate::{
    models::{
        PullRequest, PullRequestAttachment, PullRequestComment, PullRequestCommentCount,
        PullRequestCount, Repository,
    },
    requests::{
        download_pull_request_attachment::DownloadPullRequestAttachmentParams,
        get_pull_request::GetPullRequestParams,
        get_pull_request_attachment_list::GetPullRequestAttachmentListParams,
        get_pull_request_comment_count::GetPullRequestCommentCountParams,
        get_pull_request_comment_list::GetPullRequestCommentListParams,
        get_pull_request_count::GetPullRequestCountParams,
        get_pull_request_list::GetPullRequestListParams, get_repository::GetRepositoryParams,
        get_repository_list::GetRepositoryListParams,
    },
};
use backlog_api_core::Result;
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
    /// * `params` - Parameters including the project ID or key.
    pub async fn get_repository_list(
        &self,
        params: GetRepositoryListParams,
    ) -> Result<Vec<Repository>> {
        self.client.execute(params).await
    }

    /// Fetches a single Git repository by its ID or name.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including the project ID or key and repository ID or name.
    pub async fn get_repository(&self, params: GetRepositoryParams) -> Result<Repository> {
        self.client.execute(params).await
    }

    /// Fetches the list of Pull Requests for a given repository.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including the project ID or key and repository ID or name.
    pub async fn get_pull_request_list(
        &self,
        params: GetPullRequestListParams,
    ) -> Result<Vec<PullRequest>> {
        self.client.execute(params).await
    }

    /// Fetches the list of Pull Requests for a given repository with filtering options.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests`.
    ///
    /// # Arguments
    ///
    /// * `project_id_or_key` - The ID or key of the project.
    /// * `repo_id_or_name` - The ID (as a string) or name of the repository.
    /// * `params` - Parameters for filtering the pull request list.
    pub async fn get_pull_request_list_with_params(
        &self,
        params: GetPullRequestListParams,
    ) -> Result<Vec<PullRequest>> {
        self.client.execute(params).await
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
    pub async fn get_pull_request(&self, params: GetPullRequestParams) -> Result<PullRequest> {
        self.client.execute(params).await
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
        params: GetPullRequestAttachmentListParams,
    ) -> Result<Vec<PullRequestAttachment>> {
        self.client.execute(params).await
    }

    /// Downloads the content of a specific pull request attachment.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including the project ID or key, repository ID or name, pull request number, and attachment ID.
    pub async fn download_pull_request_attachment(
        &self,
        params: DownloadPullRequestAttachmentParams,
    ) -> Result<DownloadedFile> {
        self.client.download_file(params).await
    }

    /// Deletes an attachment from a pull request.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including the project ID or key, repository ID or name, pull request number, and attachment ID.
    #[cfg(feature = "writable")]
    pub async fn delete_pull_request_attachment(
        &self,
        params: DeletePullRequestAttachmentParams,
    ) -> Result<PullRequestAttachment> {
        self.client.execute(params).await
    }

    /// Fetches the list of comments for a specific pull request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including path information and optional query parameters for filtering and pagination.
    pub async fn get_pull_request_comment_list(
        &self,
        params: GetPullRequestCommentListParams,
    ) -> Result<Vec<PullRequestComment>> {
        self.client.execute(params).await
    }

    /// Fetches the count of comments for a specific pull request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/count`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including the project ID or key, repository ID or name, and pull request number.
    pub async fn get_pull_request_comment_count(
        &self,
        params: GetPullRequestCommentCountParams,
    ) -> Result<PullRequestCommentCount> {
        self.client.execute(params).await
    }

    /// Returns the count of pull requests in a repository.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/count`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including the project ID or key and repository ID or name.
    pub async fn get_pull_request_count(
        &self,
        params: GetPullRequestCountParams,
    ) -> Result<PullRequestCount> {
        self.client.execute(params).await
    }

    /// Returns the count of pull requests in a repository with filtering options.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/count`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters including the project ID or key, repository ID or name, and optional filtering parameters.
    pub async fn get_pull_request_count_with_params(
        &self,
        params: GetPullRequestListParams,
    ) -> Result<PullRequestCount> {
        self.client.execute(params).await
    }

    /// Adds a comment to a specific pull request.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for the comment including path information, content and optional user notifications.
    #[cfg(feature = "writable")]
    pub async fn add_pull_request_comment(
        &self,
        params: AddPullRequestCommentParams,
    ) -> Result<PullRequestComment> {
        self.client.execute(params).await
    }

    /// Updates a pull request.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for updating the pull request including path information and optional update fields.
    #[cfg(feature = "writable")]
    pub async fn update_pull_request(
        &self,
        params: UpdatePullRequestParams,
    ) -> Result<PullRequest> {
        self.client.execute(params).await
    }

    /// Updates a comment on a specific pull request.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/:commentId`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for updating the comment including path information and the new content.
    #[cfg(feature = "writable")]
    pub async fn update_pull_request_comment(
        &self,
        params: UpdatePullRequestCommentParams,
    ) -> Result<PullRequestComment> {
        self.client.execute(params).await
    }

    /// Creates a new pull request.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for creating the pull request including path information, summary, description, base and branch.
    #[cfg(feature = "writable")]
    pub async fn add_pull_request(&self, params: AddPullRequestParams) -> Result<PullRequest> {
        self.client.execute(params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::{ProjectIdOrKey, RepositoryIdOrName};
    // backlog_api_core::bytes is already in scope from the top-level import if we change it there.
    // No, the top level import is `backlog_api_core::bytes`, so here we'd use `bytes::Bytes`.
    // Or, import `backlog_api_core::bytes::Bytes` specifically for the test module if preferred.
    // Let's rely on the top-level `bytes` module being available.
    use crate::models::PrCommentOrder;
    #[cfg(feature = "writable")]
    use crate::requests::add_pull_request::{AddPullRequestParams, AddPullRequestParamsBuilder};
    #[cfg(feature = "writable")]
    use crate::requests::add_pull_request_comment::{
        AddPullRequestCommentParams, AddPullRequestCommentParamsBuilder,
    };
    use crate::requests::get_pull_request_count::GetPullRequestCountParamsBuilder;
    #[cfg(feature = "writable")]
    use crate::requests::update_pull_request::{
        UpdatePullRequestParams, UpdatePullRequestParamsBuilder,
    };
    #[cfg(feature = "writable")]
    use crate::requests::update_pull_request_comment::{
        UpdatePullRequestCommentParams, UpdatePullRequestCommentParamsBuilder,
    };
    use crate::requests::{
        get_pull_request_comment_list::GetPullRequestCommentListParamsBuilder,
        get_pull_request_list::GetPullRequestListParamsBuilder,
    };
    use backlog_api_core::bytes::Bytes;
    use backlog_core::identifier::{
        AttachmentId, Identifier, IssueId, PullRequestAttachmentId, PullRequestCommentId,
        PullRequestNumber, StatusId, UserId,
    };
    use client::test_utils::setup_client;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
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

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let result = git_api
            .get_pull_request_attachment_list(GetPullRequestAttachmentListParams::new(
                project_id_or_key,
                repo_id_or_name,
                pr_number,
            ))
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

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let result = git_api
            .get_pull_request_attachment_list(GetPullRequestAttachmentListParams::new(
                project_id_or_key,
                repo_id_or_name,
                pr_number,
            ))
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

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let result = git_api
            .get_pull_request_attachment_list(GetPullRequestAttachmentListParams::new(
                project_id_or_key,
                repo_id_or_name,
                pr_number,
            ))
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

        let params = DownloadPullRequestAttachmentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            attachment_id,
        );

        let result = git_api.download_pull_request_attachment(params).await;

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

        let params = DownloadPullRequestAttachmentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            attachment_id,
        );

        let result = git_api.download_pull_request_attachment(params).await;

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

        let params =
            GetPullRequestCommentListParams::new(project_id_or_key, repo_id_or_name, pr_number);

        let result = git_api.get_pull_request_comment_list(params).await;

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
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .pr_number(pr_number)
            .count(Some(1))
            .order(Some(PrCommentOrder::Asc))
            .build()
            .unwrap();

        let result = git_api.get_pull_request_comment_list(params).await;

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
        let params = AddPullRequestCommentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            "This is a test comment",
        );

        let result = git_api.add_pull_request_comment(params).await;

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
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .pr_number(pr_number)
            .content("Comment with notifications".to_string())
            .notified_user_ids(Some(vec![UserId::new(101), UserId::new(102)]))
            .build()
            .unwrap();

        let result = git_api.add_pull_request_comment(params).await;
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
        let params = AddPullRequestCommentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            "This comment should fail",
        );

        let result = git_api.add_pull_request_comment(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_comment_parameter_builder() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();
        let pr_number = PullRequestNumber::new(123);

        let params_with_all_fields = AddPullRequestCommentParamsBuilder::default()
            .project_id_or_key(project_id_or_key.clone())
            .repo_id_or_name(repo_id_or_name.clone())
            .pr_number(pr_number)
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
            .project_id_or_key(project_id_or_key.clone())
            .repo_id_or_name(repo_id_or_name.clone())
            .pr_number(pr_number)
            .content("Minimal content".to_string())
            .build()
            .unwrap();

        assert_eq!(params_minimal.content, "Minimal content");
        assert_eq!(params_minimal.notified_user_ids, None);

        let params_from_new = AddPullRequestCommentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            "From new method",
        );
        assert_eq!(params_from_new.content, "From new method");
        assert_eq!(params_from_new.notified_user_ids, None);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 131;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!({
            "id": 1001,
            "projectId": 12345,
            "repositoryId": 67890,
            "number": pr_number_val,
            "summary": "Updated PR Title",
            "description": "Updated PR Description",
            "base": "main",
            "branch": "feature/update",
            "status": {
                "id": 1,
                "name": "Open"
            },
            "assignee": {
                "id": 101,
                "userId": "testuser",
                "name": "Test User",
                "roleType": 2,
                "lang": "ja",
                "mailAddress": "test@example.com"
            },
            "issue": {
                "id": 5001
            },
            "baseCommit": "abc123",
            "branchCommit": "def456",
            "closeAt": null,
            "mergeAt": null,
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-01T10:00:00Z",
            "updatedUser": {
                "id": 101,
                "userId": "testuser",
                "name": "Test User",
                "roleType": 2,
                "lang": "ja",
                "mailAddress": "test@example.com"
            },
            "updated": "2024-01-01T14:00:00Z"
        });

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = UpdatePullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .pr_number(pr_number)
            .summary(Some("Updated PR Title".to_string()))
            .description(Some("Updated PR Description".to_string()))
            .issue_id(Some(IssueId::new(5001)))
            .assignee_id(Some(UserId::new(101)))
            .comment(Some("Updated via API".to_string()))
            .build()
            .unwrap();

        let result = git_api.update_pull_request(params).await;

        assert!(result.is_ok());
        let pull_request = result.unwrap();
        assert_eq!(pull_request.id.value(), 1001);
        assert_eq!(pull_request.summary, "Updated PR Title");
        assert_eq!(
            pull_request.description,
            Some("Updated PR Description".to_string())
        );
        assert_eq!(pull_request.assignee.as_ref().unwrap().id, UserId::new(101));
        assert_eq!(
            pull_request.related_issue.as_ref().unwrap().id,
            IssueId::new(5001)
        );
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_minimal_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 132;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!({
            "id": 1002,
            "projectId": 12345,
            "repositoryId": 67890,
            "number": pr_number_val,
            "summary": "Original Title",
            "description": "Original Description",
            "base": "main",
            "branch": "feature/test",
            "status": {
                "id": 1,
                "name": "Open"
            },
            "assignee": null,
            "issue": null,
            "baseCommit": "abc123",
            "branchCommit": "def456",
            "closeAt": null,
            "mergeAt": null,
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-01T10:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "updated": "2024-01-01T14:00:00Z"
        });

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = UpdatePullRequestParams::new(project_id_or_key, repo_id_or_name, pr_number); // All fields None

        let result = git_api.update_pull_request(params).await;

        assert!(result.is_ok());
        let pull_request = result.unwrap();
        assert_eq!(pull_request.id.value(), 1002);
        assert_eq!(pull_request.summary, "Original Title");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "NONEXISTENT";
        let repo_name = "norepo";
        let pr_number_val = 999;
        let pr_number = PullRequestNumber::new(pr_number_val);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = UpdatePullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .pr_number(pr_number)
            .summary(Some("This should fail".to_string()))
            .build()
            .unwrap();

        let result = git_api.update_pull_request(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_with_notifications() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 133;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!({
            "id": 1003,
            "projectId": 12345,
            "repositoryId": 67890,
            "number": pr_number_val,
            "summary": "PR with notifications",
            "description": "Description with notifications",
            "base": "main",
            "branch": "feature/notify",
            "status": {
                "id": 1,
                "name": "Open"
            },
            "assignee": null,
            "issue": null,
            "baseCommit": "abc123",
            "branchCommit": "def456",
            "closeAt": null,
            "mergeAt": null,
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-01T10:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "updated": "2024-01-01T15:00:00Z"
        });

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = UpdatePullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .pr_number(pr_number)
            .summary(Some("PR with notifications".to_string()))
            .description(Some("Description with notifications".to_string()))
            .notified_user_ids(Some(vec![UserId::new(201), UserId::new(202)]))
            .comment(Some("Updated with notifications".to_string()))
            .build()
            .unwrap();

        let result = git_api.update_pull_request(params).await;

        assert!(result.is_ok());
        let pull_request = result.unwrap();
        assert_eq!(pull_request.id.value(), 1003);
        assert_eq!(pull_request.summary, "PR with notifications");
        assert_eq!(
            pull_request.description,
            Some("Description with notifications".to_string())
        );
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_parameter_builder() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();
        let pr_number = PullRequestNumber::new(123);

        let params_with_all_fields = UpdatePullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key.clone())
            .repo_id_or_name(repo_id_or_name.clone())
            .pr_number(pr_number)
            .summary(Some("Test PR".to_string()))
            .description(Some("Test description".to_string()))
            .issue_id(Some(IssueId::new(1001)))
            .assignee_id(Some(UserId::new(2001)))
            .notified_user_ids(Some(vec![UserId::new(3001), UserId::new(3002)]))
            .comment(Some("Test comment".to_string()))
            .build()
            .unwrap();

        assert_eq!(params_with_all_fields.summary, Some("Test PR".to_string()));
        assert_eq!(
            params_with_all_fields.description,
            Some("Test description".to_string())
        );
        assert_eq!(params_with_all_fields.issue_id, Some(IssueId::new(1001)));
        assert_eq!(params_with_all_fields.assignee_id, Some(UserId::new(2001)));
        assert_eq!(
            params_with_all_fields.notified_user_ids,
            Some(vec![UserId::new(3001), UserId::new(3002)])
        );
        assert_eq!(
            params_with_all_fields.comment,
            Some("Test comment".to_string())
        );

        let params_minimal = UpdatePullRequestParams::new(
            project_id_or_key.clone(),
            repo_id_or_name.clone(),
            pr_number,
        );
        assert_eq!(params_minimal.summary, None);
        assert_eq!(params_minimal.description, None);
        assert_eq!(params_minimal.issue_id, None);
        assert_eq!(params_minimal.assignee_id, None);
        assert_eq!(params_minimal.notified_user_ids, None);
        assert_eq!(params_minimal.comment, None);
    }

    #[tokio::test]
    async fn test_get_pull_request_comment_count_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 134;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!({
            "count": 10
        });

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/count",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params =
            GetPullRequestCommentCountParams::new(project_id_or_key, repo_id_or_name, pr_number);
        let result = git_api.get_pull_request_comment_count(params).await;

        assert!(result.is_ok());
        let comment_count = result.unwrap();
        assert_eq!(comment_count.count, 10);
    }

    #[tokio::test]
    async fn test_get_pull_request_comment_count_zero() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 135;
        let pr_number = PullRequestNumber::new(pr_number_val);

        let mock_response = json!({
            "count": 0
        });

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/count",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params =
            GetPullRequestCommentCountParams::new(project_id_or_key, repo_id_or_name, pr_number);
        let result = git_api.get_pull_request_comment_count(params).await;

        assert!(result.is_ok());
        let comment_count = result.unwrap();
        assert_eq!(comment_count.count, 0);
    }

    #[tokio::test]
    async fn test_get_pull_request_comment_count_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "NONEXISTENT";
        let repo_name = "norepo";
        let pr_number_val = 999;
        let pr_number = PullRequestNumber::new(pr_number_val);

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/count",
                project_key, repo_name, pr_number_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params =
            GetPullRequestCommentCountParams::new(project_id_or_key, repo_id_or_name, pr_number);
        let result = git_api.get_pull_request_comment_count(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_comment_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 136;
        let pr_number = PullRequestNumber::new(pr_number_val);
        let comment_id_val = 501;
        let comment_id = PullRequestCommentId::new(comment_id_val);

        let mock_response = json!({
            "id": comment_id_val,
            "content": "Updated comment content",
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
            "updated": "2024-01-01T13:00:00Z",
            "stars": [],
            "notifications": []
        });

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/{}",
                project_key, repo_name, pr_number_val, comment_id_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = UpdatePullRequestCommentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            comment_id,
            "Updated comment content",
        );

        let result = git_api.update_pull_request_comment(params).await;

        assert!(result.is_ok());
        let comment = result.unwrap();
        assert_eq!(comment.id, PullRequestCommentId(comment_id_val));
        assert_eq!(comment.content, "Updated comment content");
        assert_eq!(comment.created_user.id, UserId(1));
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_comment_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "NONEXISTENT";
        let repo_name = "norepo";
        let pr_number_val = 999;
        let pr_number = PullRequestNumber::new(pr_number_val);
        let comment_id_val = 999;
        let comment_id = PullRequestCommentId::new(comment_id_val);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/{}",
                project_key, repo_name, pr_number_val, comment_id_val
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = UpdatePullRequestCommentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            comment_id,
            "This should fail",
        );

        let result = git_api.update_pull_request_comment(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_comment_error_403() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number_val = 137;
        let pr_number = PullRequestNumber::new(pr_number_val);
        let comment_id_val = 502;
        let comment_id = PullRequestCommentId::new(comment_id_val);

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments/{}",
                project_key, repo_name, pr_number_val, comment_id_val
            )))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let params = UpdatePullRequestCommentParams::new(
            project_id_or_key,
            repo_id_or_name,
            pr_number,
            comment_id,
            "Unauthorized update",
        );

        let result = git_api.update_pull_request_comment(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_pull_request_comment_parameter_builder() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();
        let pr_number = PullRequestNumber::new(123);
        let comment_id = PullRequestCommentId::new(456);

        let params_from_new = UpdatePullRequestCommentParams::new(
            project_id_or_key.clone(),
            repo_id_or_name.clone(),
            pr_number,
            comment_id,
            "Test content",
        );
        assert_eq!(params_from_new.content, "Test content");

        let params_from_builder = UpdatePullRequestCommentParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .pr_number(pr_number)
            .comment_id(comment_id)
            .content("Builder content".to_string())
            .build()
            .unwrap();
        assert_eq!(params_from_builder.content, "Builder content");
    }

    #[tokio::test]
    async fn test_get_pull_request_count_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";

        let mock_response = json!({
            "count": 5
        });

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/count",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = GetPullRequestCountParams::new(project_id_or_key, repo_id_or_name);
        let result = git_api.get_pull_request_count(params).await;

        assert!(result.is_ok());
        let count_response = result.unwrap();
        assert_eq!(count_response.count, 5);
    }

    #[tokio::test]
    async fn test_get_pull_request_count_error_404() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "INVALID";
        let repo_name = "norepo";

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/count",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = GetPullRequestCountParams::new(project_id_or_key, repo_id_or_name);
        let result = git_api.get_pull_request_count(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_pull_request_count_with_params_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";

        let mock_response = json!({
            "count": 2
        });

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/count",
                project_key, repo_name
            )))
            .and(query_param("statusId[]", "1"))
            .and(query_param("assigneeId[]", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = GetPullRequestCountParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .status_ids(Some(vec![1]))
            .assignee_ids(Some(vec![100]))
            .build()
            .unwrap();

        let result = git_api.get_pull_request_count(params).await;

        assert!(result.is_ok());
        let count_response = result.unwrap();
        assert_eq!(count_response.count, 2);
    }

    #[tokio::test]
    async fn test_get_pull_request_count_zero() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "empty-repo";

        let mock_response = json!({
            "count": 0
        });

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/count",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = GetPullRequestCountParams::new(project_id_or_key, repo_id_or_name);
        let result = git_api.get_pull_request_count(params).await;

        assert!(result.is_ok());
        let count_response = result.unwrap();
        assert_eq!(count_response.count, 0);
    }

    #[tokio::test]
    async fn test_get_pull_request_list_with_params_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";

        let mock_response = json!([
            {
                "id": 1,
                "projectId": 1,
                "repositoryId": 1,
                "number": 1,
                "summary": "Test PR",
                "description": "Test description",
                "base": "main",
                "branch": "feature",
                "status": {
                    "id": 1,
                    "name": "Open"
                },
                "assignee": null,
                "issue": null,
                "baseCommit": null,
                "branchCommit": null,
                "closeAt": null,
                "mergeAt": null,
                "createdUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "admin",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "created": "2024-01-01T12:00:00Z",
                "updatedUser": {
                    "id": 1,
                    "userId": "admin",
                    "name": "admin",
                    "roleType": 1,
                    "lang": "ja",
                    "mailAddress": "admin@example.com"
                },
                "updated": "2024-01-01T12:00:00Z"
            }
        ]);

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests",
                project_key, repo_name
            )))
            .and(query_param("statusId[]", "1"))
            .and(query_param("count", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = GetPullRequestListParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .status_ids(vec![StatusId::new(1)])
            .count(10)
            .build()
            .unwrap();

        let result = git_api.get_pull_request_list(params).await;

        assert!(result.is_ok());
        let prs = result.unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].number.value(), 1);
        assert_eq!(prs[0].summary, "Test PR");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_minimal_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";

        let mock_response = json!({
            "id": 123,
            "projectId": 1,
            "repositoryId": 1,
            "number": 5,
            "summary": "Fix authentication bug",
            "description": "This PR fixes the authentication issue",
            "base": "main",
            "branch": "feature/fix-auth",
            "status": {
                "id": 1,
                "name": "Open"
            },
            "assignee": null,
            "issue": null,
            "baseCommit": null,
            "branchCommit": null,
            "closeAt": null,
            "mergeAt": null,
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-01T12:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "updated": "2024-01-01T12:00:00Z"
        });

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = AddPullRequestParams::new(
            project_id_or_key,
            repo_id_or_name,
            "Fix authentication bug",
            "This PR fixes the authentication issue",
            "main",
            "feature/fix-auth",
        );

        let result = git_api.add_pull_request(params).await;

        assert!(result.is_ok());
        let pr = result.unwrap();
        assert_eq!(pr.number.value(), 5);
        assert_eq!(pr.summary, "Fix authentication bug");
        assert_eq!(
            pr.description,
            Some("This PR fixes the authentication issue".to_string())
        );
        assert_eq!(pr.base, "main");
        assert_eq!(pr.branch, "feature/fix-auth");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_full_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";

        let mock_response = json!({
            "id": 124,
            "projectId": 1,
            "repositoryId": 1,
            "number": 6,
            "summary": "Add new feature",
            "description": "This PR adds a new feature with comprehensive tests",
            "base": "develop",
            "branch": "feature/new-feature",
            "status": {
                "id": 1,
                "name": "Open"
            },
            "assignee": {
                "id": 456,
                "userId": "developer",
                "name": "Developer",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "developer@example.com"
            },
            "issue": {
                "id": 123,
                "issueKey": "PROJ-123",
                "keyId": 123,
                "summary": "Implement new feature"
            },
            "baseCommit": null,
            "branchCommit": null,
            "closeAt": null,
            "mergeAt": null,
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-01T12:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "admin",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "updated": "2024-01-01T12:00:00Z"
        });

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = AddPullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .summary("Add new feature".to_string())
            .description("This PR adds a new feature with comprehensive tests".to_string())
            .base("develop".to_string())
            .branch("feature/new-feature".to_string())
            .issue_id(IssueId::new(123))
            .assignee_id(UserId::new(456))
            .notified_user_ids(vec![UserId::new(789), UserId::new(101112)])
            .attachment_ids(vec![AttachmentId::new(111), AttachmentId::new(222)])
            .build()
            .unwrap();

        let result = git_api.add_pull_request(params).await;

        assert!(result.is_ok());
        let pr = result.unwrap();
        assert_eq!(pr.number.value(), 6);
        assert_eq!(pr.summary, "Add new feature");
        assert_eq!(pr.base, "develop");
        assert_eq!(pr.branch, "feature/new-feature");
        assert!(pr.assignee.is_some());
        assert_eq!(pr.assignee.unwrap().id.value(), 456);
        assert!(pr.related_issue.is_some());
        assert_eq!(pr.related_issue.unwrap().id.value(), 123);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "INVALID";
        let repo_name = "test-repo";

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = AddPullRequestParams::new(
            project_id_or_key,
            repo_id_or_name,
            "Test PR",
            "Test description",
            "main",
            "feature/test",
        );

        let result = git_api.add_pull_request(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_repository_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "invalid-repo";

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = AddPullRequestParams::new(
            project_id_or_key,
            repo_id_or_name,
            "Test PR",
            "Test description",
            "main",
            "feature/test",
        );

        let result = git_api.add_pull_request(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_permission_denied() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "RESTRICTED";
        let repo_name = "test-repo";

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = AddPullRequestParams::new(
            project_id_or_key,
            repo_id_or_name,
            "Test PR",
            "Test description",
            "main",
            "feature/test",
        );

        let result = git_api.add_pull_request(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_invalid_branch() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";

        let mock_error_response = json!({
            "errors": [
                {
                    "message": "Branch 'nonexistent-branch' does not exist",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests",
                project_key, repo_name
            )))
            .respond_with(ResponseTemplate::new(400).set_body_json(&mock_error_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();

        let params = AddPullRequestParams::new(
            project_id_or_key,
            repo_id_or_name,
            "Test PR",
            "Test description",
            "main",
            "nonexistent-branch",
        );

        let result = git_api.add_pull_request(params).await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_pull_request_parameter_builder() {
        let project_id_or_key: ProjectIdOrKey = "TEST".parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = "test-repo".parse().unwrap();

        let params_from_new = AddPullRequestParams::new(
            project_id_or_key.clone(),
            repo_id_or_name.clone(),
            "Test title",
            "Test description",
            "main",
            "feature/test",
        );
        assert_eq!(params_from_new.summary, "Test title");
        assert_eq!(params_from_new.description, "Test description");
        assert_eq!(params_from_new.base, "main");
        assert_eq!(params_from_new.branch, "feature/test");
        assert!(params_from_new.issue_id.is_none());

        let params_from_builder = AddPullRequestParamsBuilder::default()
            .project_id_or_key(project_id_or_key)
            .repo_id_or_name(repo_id_or_name)
            .summary("Builder title".to_string())
            .description("Builder description".to_string())
            .base("develop".to_string())
            .branch("feature/builder".to_string())
            .issue_id(IssueId::new(456))
            .build()
            .unwrap();
        assert_eq!(params_from_builder.summary, "Builder title");
        assert_eq!(params_from_builder.issue_id, Some(IssueId::new(456)));
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_pull_request_attachment_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number = 1u64;
        let attachment_id = 123u32;

        let mock_response = json!({
            "id": 123,
            "name": "test-file.txt",
            "size": 1024
        });

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
                project_key, repo_name, pr_number, attachment_id
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let pr_num = PullRequestNumber::from(pr_number);
        let attach_id = PullRequestAttachmentId::new(attachment_id);

        let result = git_api
            .delete_pull_request_attachment(DeletePullRequestAttachmentParams::new(
                project_id_or_key,
                repo_id_or_name,
                pr_num,
                attach_id,
            ))
            .await;

        assert!(result.is_ok());
        let attachment = result.unwrap();
        assert_eq!(attachment.id, PullRequestAttachmentId::new(123));
        assert_eq!(attachment.name, "test-file.txt");
        assert_eq!(attachment.size, 1024);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_pull_request_attachment_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number = 1u64;
        let attachment_id = 999u32;

        let mock_error_response = json!({
            "errors": [
                {
                    "message": "Attachment not found",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
                project_key, repo_name, pr_number, attachment_id
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(&mock_error_response))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let pr_num = PullRequestNumber::from(pr_number);
        let attach_id = PullRequestAttachmentId::new(attachment_id);

        let result = git_api
            .delete_pull_request_attachment(DeletePullRequestAttachmentParams::new(
                project_id_or_key,
                repo_id_or_name,
                pr_num,
                attach_id,
            ))
            .await;

        assert!(result.is_err());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_pull_request_attachment_permission_denied() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let git_api = GitApi::new(client);

        let project_key = "RESTRICTED";
        let repo_name = "test-repo";
        let pr_number = 1u64;
        let attachment_id = 123u32;

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments/{}",
                project_key, repo_name, pr_number, attachment_id
            )))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let project_id_or_key: ProjectIdOrKey = project_key.parse().unwrap();
        let repo_id_or_name: RepositoryIdOrName = repo_name.parse().unwrap();
        let pr_num = PullRequestNumber::from(pr_number);
        let attach_id = PullRequestAttachmentId::new(attachment_id);

        let result = git_api
            .delete_pull_request_attachment(DeletePullRequestAttachmentParams::new(
                project_id_or_key,
                repo_id_or_name,
                pr_num,
                attach_id,
            ))
            .await;

        assert!(result.is_err());
    }
}
