use backlog_api_core::Result;
use client::Client;

use crate::api::{
    GetSpaceDiskUsageParams, GetSpaceDiskUsageResponse, GetSpaceLogoParams, GetSpaceLogoResponse,
    GetSpaceParams, GetSpaceResponse,
};
#[cfg(feature = "writable")]
use crate::api::{UploadAttachmentParams, UploadAttachmentResponse};

pub struct SpaceApi(Client);

impl SpaceApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get space information
    /// Corresponds to `GET /api/v2/space`.
    pub async fn get_space(&self, params: GetSpaceParams) -> Result<GetSpaceResponse> {
        self.0.execute(params).await
    }

    /// Get space logo
    /// Corresponds to `GET /api/v2/space/image`.
    pub async fn get_space_logo(&self, params: GetSpaceLogoParams) -> Result<GetSpaceLogoResponse> {
        self.0.download_file(params).await
    }

    /// Get space disk usage
    /// Corresponds to `GET /api/v2/space/diskUsage`.
    pub async fn get_space_disk_usage(
        &self,
        params: GetSpaceDiskUsageParams,
    ) -> Result<GetSpaceDiskUsageResponse> {
        self.0.execute(params).await
    }

    /// Upload an attachment file
    /// Corresponds to `POST /api/v2/space/attachment`.
    #[cfg(feature = "writable")]
    pub async fn upload_attachment(
        &self,
        params: UploadAttachmentParams,
    ) -> Result<UploadAttachmentResponse> {
        self.0.upload_file(params).await
    }
}
