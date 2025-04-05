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
}
