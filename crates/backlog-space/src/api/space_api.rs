use backlog_api_core::Result;
use client::Client;

use crate::api::{
    GetLicenceParams, GetLicenceResponse, GetSpaceDiskUsageParams, GetSpaceDiskUsageResponse,
    GetSpaceLogoParams, GetSpaceLogoResponse, GetSpaceNotificationParams,
    GetSpaceNotificationResponse, GetSpaceParams, GetSpaceResponse,
};
#[cfg(feature = "writable")]
use crate::api::{
    UpdateSpaceNotificationParams, UpdateSpaceNotificationResponse, UploadAttachmentParams,
    UploadAttachmentResponse,
};

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

    /// Get licence information
    /// Corresponds to `GET /api/v2/space/licence`.
    pub async fn get_licence(&self, params: GetLicenceParams) -> Result<GetLicenceResponse> {
        self.0.execute(params).await
    }

    /// Get space notification
    /// Corresponds to `GET /api/v2/space/notification`.
    pub async fn get_space_notification(
        &self,
        params: GetSpaceNotificationParams,
    ) -> Result<GetSpaceNotificationResponse> {
        self.0.execute(params).await
    }

    /// Update space notification
    /// Corresponds to `PUT /api/v2/space/notification`.
    #[cfg(feature = "writable")]
    pub async fn update_space_notification(
        &self,
        params: UpdateSpaceNotificationParams,
    ) -> Result<UpdateSpaceNotificationResponse> {
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
