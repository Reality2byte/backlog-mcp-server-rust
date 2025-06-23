use super::{GetFileParams, GetSharedFilesListParams, GetSharedFilesListResponse};
use backlog_api_core::Result;
use backlog_core::{ProjectIdOrKey, identifier::SharedFileId};
use client::{Client, DownloadedFile};

pub struct FileApi(Client);

impl FileApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Gets the list of shared files for a project directory.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/files/metadata/:path`.
    pub async fn get_shared_files_list(
        &self,
        params: GetSharedFilesListParams,
    ) -> Result<GetSharedFilesListResponse> {
        self.0.execute(params).await
    }

    /// Downloads a shared file by its ID.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/files/:sharedFileId`.
    pub async fn get_file(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        shared_file_id: SharedFileId,
    ) -> Result<DownloadedFile> {
        let params = GetFileParams::new(project_id_or_key, shared_file_id);
        self.0.download_file(params).await
    }
}
