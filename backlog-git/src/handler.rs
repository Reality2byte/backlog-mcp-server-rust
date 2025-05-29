//! Handler for Git related API endpoints.

use client::Client; // The generic HTTP client from the `client` crate
use crate::models::{PullRequest, Repository};
use crate::error::Result; // This crate's Result type
use backlog_core::ProjectIdOrKey;

/// Provides access to the Git and Pull Request related API functions.
#[derive(Debug, Clone)]
pub struct GitHandler {
    client: Client,
}

impl GitHandler {
    /// Creates a new GitHandler.
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
    pub async fn list_repositories(
        &self,
        project_id_or_key: &ProjectIdOrKey,
    ) -> Result<Vec<Repository>> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories",
            project_id_or_key
        );
        self.client.get(&path).await.map_err(Into::into)
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
        project_id_or_key: &ProjectIdOrKey,
        repo_id_or_name: &str,
    ) -> Result<Repository> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}",
            project_id_or_key, repo_id_or_name
        );
        self.client.get(&path).await.map_err(Into::into)
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
    pub async fn list_pull_requests(
        &self,
        project_id_or_key: &ProjectIdOrKey,
        repo_id_or_name: &str,
    ) -> Result<Vec<PullRequest>> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests",
            project_id_or_key, repo_id_or_name
        );
        // For now, no query parameters. If params were added:
        // self.client.get_with_params(&path, params_struct_ref).await.map_err(Into::into)
        self.client.get(&path).await.map_err(Into::into)
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
        project_id_or_key: &ProjectIdOrKey,
        repo_id_or_name: &str,
        pr_number: u64,
    ) -> Result<PullRequest> {
        let path = format!(
            "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}",
            project_id_or_key, repo_id_or_name, pr_number
        );
        self.client.get(&path).await.map_err(Into::into)
    }

    // TODO:
    // - (Consider request parameter structs like ListPullRequestsParams for query options for list_pull_requests)
    // - Consider creating a RepositoryIdOrName type in backlog-core for repo_id_or_name.
    // - Implement functions for PR comments and attachments if needed.
    // - Implement functions for creating/updating PRs if writable features are desired.
}
