use backlog_api_core::Result;
use client::Client;

use crate::api::{GetSpaceLogoParams, GetSpaceLogoResponse, GetSpaceParams, GetSpaceResponse};

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
}
