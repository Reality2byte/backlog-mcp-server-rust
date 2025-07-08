#[cfg(feature = "writable")]
use crate::api::{
    AddRecentlyViewedWikiParams, AddRecentlyViewedWikiResponse, AddWikiParams, AddWikiResponse,
    AttachFilesToWikiParams, AttachFilesToWikiResponse, DeleteWikiAttachmentParams,
    DeleteWikiAttachmentResponse, DeleteWikiParams, DeleteWikiResponse,
    LinkSharedFilesToWikiParams, LinkSharedFilesToWikiResponse, UnlinkSharedFileFromWikiParams,
    UnlinkSharedFileFromWikiResponse, UpdateWikiParams, UpdateWikiResponse,
};
use crate::api::{
    DownloadWikiAttachmentParams, GetRecentlyViewedWikisParams, GetRecentlyViewedWikisResponse,
    GetWikiAttachmentListParams, GetWikiAttachmentListResponse, GetWikiCountParams,
    GetWikiCountResponse, GetWikiDetailParams, GetWikiDetailResponse, GetWikiHistoryParams,
    GetWikiHistoryResponse, GetWikiListParams, GetWikiListResponse, GetWikiSharedFileListParams,
    GetWikiSharedFileListResponse, GetWikiStarsParams, GetWikiStarsResponse, GetWikiTagListParams,
    GetWikiTagListResponse,
};
use backlog_api_core::Result;
use client::Client;

pub struct WikiApi(Client);

impl WikiApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    // Read operations
    /// Get wiki page count
    /// Corresponds to `GET /api/v2/wikis/count`.
    pub async fn get_wiki_count(&self, params: GetWikiCountParams) -> Result<GetWikiCountResponse> {
        self.0.execute(params).await
    }

    /// Get wiki page details
    /// Corresponds to `GET /api/v2/wikis/:wikiId`.
    pub async fn get_wiki_detail(
        &self,
        params: GetWikiDetailParams,
    ) -> Result<GetWikiDetailResponse> {
        self.0.execute(params).await
    }

    /// Get wiki page list
    /// Corresponds to `GET /api/v2/wikis`.
    pub async fn get_wiki_list(&self, params: GetWikiListParams) -> Result<GetWikiListResponse> {
        self.0.execute(params).await
    }

    /// Get list of tags used in wiki pages within a project.
    /// Corresponds to `GET /api/v2/wikis/tags`.
    pub async fn get_wiki_tag_list(
        &self,
        params: GetWikiTagListParams,
    ) -> Result<GetWikiTagListResponse> {
        self.0.execute(params).await
    }

    /// Get wiki page history
    /// Corresponds to `GET /api/v2/wikis/:wikiId/history`.
    pub async fn get_wiki_history(
        &self,
        params: GetWikiHistoryParams,
    ) -> Result<GetWikiHistoryResponse> {
        self.0.execute(params).await
    }

    /// Get wiki attachment list
    /// Corresponds to `GET /api/v2/wikis/:wikiId/attachments`.
    pub async fn get_wiki_attachment_list(
        &self,
        params: GetWikiAttachmentListParams,
    ) -> Result<GetWikiAttachmentListResponse> {
        self.0.execute(params).await
    }

    /// Download wiki attachment
    /// Corresponds to `GET /api/v2/wikis/:wikiId/attachments/:attachmentId`.
    pub async fn download_wiki_attachment(
        &self,
        params: DownloadWikiAttachmentParams,
    ) -> Result<client::DownloadedFile> {
        self.0.download_file(params).await
    }

    /// Get wiki shared file list
    /// Corresponds to `GET /api/v2/wikis/:wikiId/sharedFiles`.
    pub async fn get_wiki_shared_file_list(
        &self,
        params: GetWikiSharedFileListParams,
    ) -> Result<GetWikiSharedFileListResponse> {
        self.0.execute(params).await
    }

    /// Get the list of stars received by a wiki page.
    /// Corresponds to `GET /api/v2/wikis/:wikiId/stars`.
    pub async fn get_wiki_stars(&self, params: GetWikiStarsParams) -> Result<GetWikiStarsResponse> {
        self.0.execute(params).await
    }

    /// Get recently viewed wikis
    /// Corresponds to `GET /api/v2/users/myself/recentlyViewedWikis`.
    pub async fn get_recently_viewed_wikis(
        &self,
        params: GetRecentlyViewedWikisParams,
    ) -> Result<GetRecentlyViewedWikisResponse> {
        self.0.execute(params).await
    }

    /// Add recently viewed wiki
    /// Corresponds to `POST /api/v2/users/myself/recentlyViewedWikis`.
    #[cfg(feature = "writable")]
    pub async fn add_recently_viewed_wiki(
        &self,
        params: AddRecentlyViewedWikiParams,
    ) -> Result<AddRecentlyViewedWikiResponse> {
        self.0.execute(params).await
    }

    /// Add new wiki page
    /// Corresponds to `POST /api/v2/wikis`.
    #[cfg(feature = "writable")]
    pub async fn add_wiki(&self, params: AddWikiParams) -> Result<AddWikiResponse> {
        self.0.execute(params).await
    }

    /// Update wiki page
    /// Corresponds to `PATCH /api/v2/wikis/:wikiId`.
    #[cfg(feature = "writable")]
    pub async fn update_wiki(&self, params: UpdateWikiParams) -> Result<UpdateWikiResponse> {
        self.0.execute(params).await
    }

    /// Delete wiki page
    /// Corresponds to `DELETE /api/v2/wikis/:wikiId`.
    #[cfg(feature = "writable")]
    pub async fn delete_wiki(&self, params: DeleteWikiParams) -> Result<DeleteWikiResponse> {
        self.0.execute(params).await
    }

    /// Attach files to wiki page
    /// Corresponds to `POST /api/v2/wikis/:wikiId/attachments`.
    #[cfg(feature = "writable")]
    pub async fn attach_files_to_wiki(
        &self,
        params: AttachFilesToWikiParams,
    ) -> Result<AttachFilesToWikiResponse> {
        self.0.execute(params).await
    }

    /// Delete wiki attachment
    /// Corresponds to `DELETE /api/v2/wikis/:wikiId/attachments/:attachmentId`.
    #[cfg(feature = "writable")]
    pub async fn delete_wiki_attachment(
        &self,
        params: DeleteWikiAttachmentParams,
    ) -> Result<DeleteWikiAttachmentResponse> {
        self.0.execute(params).await
    }

    /// Link shared files to wiki page
    /// Corresponds to `POST /api/v2/wikis/:wikiId/sharedFiles`.
    #[cfg(feature = "writable")]
    pub async fn link_shared_files_to_wiki(
        &self,
        params: LinkSharedFilesToWikiParams,
    ) -> Result<LinkSharedFilesToWikiResponse> {
        self.0.execute(params).await
    }

    /// Unlink shared file from wiki page
    /// Corresponds to `DELETE /api/v2/wikis/:wikiId/sharedFiles/:id`.
    #[cfg(feature = "writable")]
    pub async fn unlink_shared_file_from_wiki(
        &self,
        params: UnlinkSharedFileFromWikiParams,
    ) -> Result<UnlinkSharedFileFromWikiResponse> {
        self.0.execute(params).await
    }
}
