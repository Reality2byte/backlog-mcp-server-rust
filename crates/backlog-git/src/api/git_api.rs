use backlog_api_core::Result;
use client::Client;

use super::{
    DownloadPullRequestAttachmentParams, DownloadPullRequestAttachmentResponse,
    GetPullRequestAttachmentListParams, GetPullRequestAttachmentListResponse,
    GetPullRequestCommentCountParams, GetPullRequestCommentCountResponse,
    GetPullRequestCommentListParams, GetPullRequestCommentListResponse, GetPullRequestCountParams,
    GetPullRequestCountResponse, GetPullRequestListParams, GetPullRequestListResponse,
    GetPullRequestParams, GetPullRequestResponse, GetRepositoryListParams,
    GetRepositoryListResponse, GetRepositoryParams, GetRepositoryResponse,
};

#[cfg(feature = "writable")]
use super::{
    AddPullRequestCommentParams, AddPullRequestCommentResponse, AddPullRequestParams,
    AddPullRequestResponse, DeletePullRequestAttachmentParams, DeletePullRequestAttachmentResponse,
    UpdatePullRequestCommentParams, UpdatePullRequestCommentResponse, UpdatePullRequestParams,
    UpdatePullRequestResponse,
};

/// Provides access to the Git and Pull Request related API functions.
#[derive(Debug, Clone)]
pub struct GitApi(Client);

impl GitApi {
    /// Creates a new GitApi.
    ///
    /// This is typically called by `BacklogApiClient::git()`.
    ///
    /// # Arguments
    ///
    /// * `client` - An instance of the generic `client::Client`.
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Fetches the list of Git repositories for a given project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories`.
    pub async fn get_repository_list(
        &self,
        params: GetRepositoryListParams,
    ) -> Result<GetRepositoryListResponse> {
        self.0.execute(params).await
    }

    /// Fetches a single Git repository by its ID or name.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName`.
    pub async fn get_repository(
        &self,
        params: GetRepositoryParams,
    ) -> Result<GetRepositoryResponse> {
        self.0.execute(params).await
    }

    /// Fetches the list of Pull Requests for a given repository.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests`.
    pub async fn get_pull_request_list(
        &self,
        params: GetPullRequestListParams,
    ) -> Result<GetPullRequestListResponse> {
        self.0.execute(params).await
    }

    /// Fetches a single Pull Request by its number.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number`.
    pub async fn get_pull_request(
        &self,
        params: GetPullRequestParams,
    ) -> Result<GetPullRequestResponse> {
        self.0.execute(params).await
    }

    /// Fetches the count of Pull Requests in a repository.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/count`.
    pub async fn get_pull_request_count(
        &self,
        params: GetPullRequestCountParams,
    ) -> Result<GetPullRequestCountResponse> {
        self.0.execute(params).await
    }

    /// Fetches the list of comments for a Pull Request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments`.
    pub async fn get_pull_request_comment_list(
        &self,
        params: GetPullRequestCommentListParams,
    ) -> Result<GetPullRequestCommentListResponse> {
        self.0.execute(params).await
    }

    /// Fetches the count of comments for a Pull Request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/count`.
    pub async fn get_pull_request_comment_count(
        &self,
        params: GetPullRequestCommentCountParams,
    ) -> Result<GetPullRequestCommentCountResponse> {
        self.0.execute(params).await
    }

    /// Fetches the list of attachments for a Pull Request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments`.
    pub async fn get_pull_request_attachment_list(
        &self,
        params: GetPullRequestAttachmentListParams,
    ) -> Result<GetPullRequestAttachmentListResponse> {
        self.0.execute(params).await
    }

    /// Downloads an attachment from a Pull Request.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId`.
    pub async fn download_pull_request_attachment(
        &self,
        params: DownloadPullRequestAttachmentParams,
    ) -> Result<DownloadPullRequestAttachmentResponse> {
        self.0.download_file(params).await
    }

    /// Creates a new Pull Request.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests`.
    #[cfg(feature = "writable")]
    pub async fn add_pull_request(
        &self,
        params: AddPullRequestParams,
    ) -> Result<AddPullRequestResponse> {
        self.0.execute(params).await
    }

    /// Adds a comment to a Pull Request.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments`.
    #[cfg(feature = "writable")]
    pub async fn add_pull_request_comment(
        &self,
        params: AddPullRequestCommentParams,
    ) -> Result<AddPullRequestCommentResponse> {
        self.0.execute(params).await
    }

    /// Updates a Pull Request.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number`.
    #[cfg(feature = "writable")]
    pub async fn update_pull_request(
        &self,
        params: UpdatePullRequestParams,
    ) -> Result<UpdatePullRequestResponse> {
        self.0.execute(params).await
    }

    /// Updates a Pull Request comment.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/:commentId`.
    #[cfg(feature = "writable")]
    pub async fn update_pull_request_comment(
        &self,
        params: UpdatePullRequestCommentParams,
    ) -> Result<UpdatePullRequestCommentResponse> {
        self.0.execute(params).await
    }

    /// Deletes an attachment from a Pull Request.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId`.
    #[cfg(feature = "writable")]
    pub async fn delete_pull_request_attachment(
        &self,
        params: DeletePullRequestAttachmentParams,
    ) -> Result<DeletePullRequestAttachmentResponse> {
        self.0.execute(params).await
    }
}
