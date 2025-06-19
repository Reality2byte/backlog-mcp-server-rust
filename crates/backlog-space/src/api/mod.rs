use backlog_api_core::Result;
use client::Client;

use crate::GetSpaceResponse;

pub struct SpaceApi(Client);

impl SpaceApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_space(&self) -> Result<GetSpaceResponse> {
        self.0.get("/api/v2/space").await
    }

    pub async fn get_space_logo(&self) -> Result<Vec<u8>> {
        let downloaded_file = self.0.download_file_raw("/api/v2/space/image").await?;
        Ok(downloaded_file.bytes.to_vec())
    }
}
