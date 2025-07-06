use crate::{GetRateLimitParams, GetRateLimitResponse};
use backlog_api_core::Result;
use client::Client;

pub struct RateLimitApi(Client);

impl RateLimitApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Gets the rate limit information for the API key.
    ///
    /// Corresponds to `GET /api/v2/rateLimit`.
    pub async fn get_rate_limit(&self) -> Result<GetRateLimitResponse> {
        self.0.execute(GetRateLimitParams::new()).await
    }
}
