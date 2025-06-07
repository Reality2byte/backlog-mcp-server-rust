use crate::models::{PullRequest, PullRequestAttachment, Repository};
use backlog_api_core::Result;
use backlog_core::{ProjectIdOrKey, RepositoryIdOrName};
use client::Client; // The generic HTTP client from the `client` crate

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
        pr_number: u64,
    ) -> Result<PullRequest> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
            project_id_or_key.into(),
            repo_id_or_name.into(),
            pr_number
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
        project_id_or_key: &ProjectIdOrKey,
        repo_id_or_name: &RepositoryIdOrName,
        pr_number: u64,
    ) -> backlog_api_core::Result<Vec<PullRequestAttachment>> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments", // "/git/" を追加
            project_id_or_key, // Display trait を利用
            repo_id_or_name,   // Display trait を利用
            pr_number
        );
        self.client.get(&path).await // クエリパラメータはなし
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::{AttachmentId, Identifier};
    use client::test_utils::setup_client; // Identifier を追加
    // use std::str::FromStr; // FromStr を削除 (parse() メソッド呼び出しのため不要)
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_pull_request_attachment_list_success_multiple_attachments() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await; // server.uri().as_str() から &server に変更
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number = 123;

        let mock_response = vec![
            PullRequestAttachment {
                id: AttachmentId::new(101),
                name: "image.png".to_string(),
                size: 12345,
            },
            PullRequestAttachment {
                id: AttachmentId::new(102),
                name: "document.pdf".to_string(),
                size: 67890,
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments", // "/git/" を追加
                project_key, repo_name, pr_number
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
        let client = setup_client(&server).await; // server.uri().as_str() から &server に変更
        let git_api = GitApi::new(client);

        let project_key = "TESTPROJECT";
        let repo_name = "test-repo";
        let pr_number = 124;
        let mock_response: Vec<PullRequestAttachment> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments", // "/git/" を追加
                project_key, repo_name, pr_number
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
        let client = setup_client(&server).await; // server.uri().as_str() から &server に変更
        let git_api = GitApi::new(client);

        let project_key = "NONEXISTENT";
        let repo_name = "norepo";
        let pr_number = 1;

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/attachments", // "/git/" を追加
                project_key, repo_name, pr_number
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
}
