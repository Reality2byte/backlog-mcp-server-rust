use backlog_api_core::Result;
use client::{Client, DownloadedFile};

#[cfg(feature = "writable")]
use super::{
    AddCommentParams, AddIssueParams, DeleteIssueParams, LinkSharedFilesToIssueParams,
    UpdateIssueParams,
};
#[cfg(feature = "writable")]
use super::{
    AddCommentResponse, AddIssueResponse, DeleteIssueResponse, LinkSharedFilesToIssueResponse,
    UpdateIssueResponse,
};
use super::{
    CountCommentParams, CountIssueParams, GetAttachmentFileParams, GetAttachmentListParams,
    GetCommentListParams, GetCommentParams, GetIssueListParams, GetIssueParams,
    GetSharedFileListParams,
};
use super::{
    CountCommentResponse, CountIssueResponse, GetAttachmentListResponse, GetCommentListResponse,
    GetCommentResponse, GetIssueListResponse, GetIssueResponse, GetSharedFileListResponse,
};

pub struct IssueApi(Client);

impl IssueApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get issue by its ID or key.
    pub async fn get_issue(&self, params: GetIssueParams) -> Result<GetIssueResponse> {
        self.0.execute(params).await
    }

    /// Get a list of issues with optional parameters.
    pub async fn get_issue_list(&self, params: GetIssueListParams) -> Result<GetIssueListResponse> {
        self.0.execute(params).await
    }

    /// Count issues based on the provided parameters.
    pub async fn count_issue(&self, params: CountIssueParams) -> Result<CountIssueResponse> {
        self.0.execute(params).await
    }

    /// Add a new issue with the provided parameters.
    #[cfg(feature = "writable")]
    pub async fn add_issue(&self, params: AddIssueParams) -> Result<AddIssueResponse> {
        self.0.execute(params).await
    }

    /// Delete an issue by its key.
    #[cfg(feature = "writable")]
    pub async fn delete_issue(&self, params: DeleteIssueParams) -> Result<DeleteIssueResponse> {
        self.0.execute(params).await
    }

    /// Update an existing issue by its ID or key.
    #[cfg(feature = "writable")]
    pub async fn update_issue(&self, params: UpdateIssueParams) -> Result<UpdateIssueResponse> {
        self.0.execute(params).await
    }

    /// Add a new comment to an existing issue.
    #[cfg(feature = "writable")]
    pub async fn add_comment(&self, params: AddCommentParams) -> Result<AddCommentResponse> {
        self.0.execute(params).await
    }

    /// Get a list of comments for an issue by its ID or key.
    pub async fn get_comment_list(
        &self,
        params: GetCommentListParams,
    ) -> Result<GetCommentListResponse> {
        self.0.execute(params).await
    }

    /// Count comments for an issue by its ID or key.
    pub async fn count_comment(&self, params: CountCommentParams) -> Result<CountCommentResponse> {
        self.0.execute(params).await
    }

    /// Get a specific comment for an issue by its ID or key and comment ID.
    pub async fn get_comment(&self, params: GetCommentParams) -> Result<GetCommentResponse> {
        self.0.execute(params).await
    }

    /// Get a list of attachments for an issue by its ID or key.
    pub async fn get_attachment_list(
        &self,
        params: GetAttachmentListParams,
    ) -> Result<GetAttachmentListResponse> {
        self.0.execute(params).await
    }

    /// Get a list of shared files linked to an issue.
    ///
    /// Corresponds to `GET /api/v2/issues/:issueIdOrKey/sharedFiles`.
    pub async fn get_shared_file_list(
        &self,
        params: GetSharedFileListParams,
    ) -> Result<GetSharedFileListResponse> {
        self.0.execute(params).await
    }

    /// Link shared files to an issue.
    ///
    /// Corresponds to `POST /api/v2/issues/:issueIdOrKey/sharedFiles`.
    #[cfg(feature = "writable")]
    pub async fn link_shared_files_to_issue(
        &self,
        params: LinkSharedFilesToIssueParams,
    ) -> Result<LinkSharedFilesToIssueResponse> {
        self.0.execute(params).await
    }

    /// Get a specific attachment file by issue ID or key and attachment ID.
    pub async fn get_attachment_file(
        &self,
        params: GetAttachmentFileParams,
    ) -> backlog_api_core::Result<DownloadedFile> {
        self.0.download_file(params).await
    }
}
