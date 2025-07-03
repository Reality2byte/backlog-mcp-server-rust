use backlog_api_core::Result;
use backlog_core::identifier::ActivityId;
use client::Client;

use super::get_activity::{GetActivityParams, GetActivityResponse};

#[derive(Clone, Debug)]
pub struct ActivityApi(Client);

impl ActivityApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get an activity by ID.
    /// Corresponds to `GET /api/v2/activities/:activityId`.
    pub async fn get_activity(
        &self,
        activity_id: impl Into<ActivityId>,
    ) -> Result<GetActivityResponse> {
        let params = GetActivityParams {
            activity_id: activity_id.into(),
        };
        self.0.execute(params).await
    }
}
