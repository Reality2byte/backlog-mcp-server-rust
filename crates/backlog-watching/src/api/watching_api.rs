use backlog_api_core::Result;
use backlog_core::identifier::WatchingId;
use client::Client;

use crate::api::get_watching::{GetWatchingParams, GetWatchingResponse};

#[cfg(feature = "writable")]
use crate::api::{
    add_watching::{AddWatchingParams, AddWatchingResponse},
    delete_watching::{DeleteWatchingParams, DeleteWatchingResponse},
    mark_as_read::MarkAsReadParams,
    update_watching::{UpdateWatchingParams, UpdateWatchingResponse},
};

/// API client for Backlog Watching operations.
///
/// Provides methods to interact with watching-related endpoints in the Backlog API.
#[derive(Debug, Clone)]
pub struct WatchingApi {
    client: Client,
}

impl WatchingApi {
    /// Creates a new instance of `WatchingApi`.
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Gets a specific watching by its ID.
    ///
    /// Corresponds to `GET /api/v2/watchings/:watchingId`.
    ///
    /// # Arguments
    ///
    /// * `watching_id` - The ID of the watching to retrieve
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use backlog_watching::WatchingApi;
    /// # use backlog_core::identifier::WatchingId;
    /// # async fn example(api: WatchingApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let watching = api.get(WatchingId::from(123)).await?;
    /// println!("Watching note: {:?}", watching.note);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, watching_id: impl Into<WatchingId>) -> Result<GetWatchingResponse> {
        let params = GetWatchingParams::new(watching_id);
        self.client.execute(params).await
    }

    /// Adds a new watching for an issue.
    ///
    /// Corresponds to `POST /api/v2/watchings`.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for adding the watching
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use backlog_watching::{WatchingApi, AddWatchingParams};
    /// # use backlog_core::IssueIdOrKey;
    /// # use std::str::FromStr;
    /// # async fn example(api: WatchingApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = AddWatchingParams::new(IssueIdOrKey::Key(backlog_core::IssueKey::from_str("PROJ-123")?))
    ///     .with_note("Important to track");
    /// let watching = api.add(params).await?;
    /// println!("Created watching with ID: {}", watching.id);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "writable")]
    pub async fn add(&self, params: AddWatchingParams) -> Result<AddWatchingResponse> {
        self.client.execute(params).await
    }

    /// Updates an existing watching.
    ///
    /// Corresponds to `PATCH /api/v2/watchings/:watchingId`.
    ///
    /// # Arguments
    ///
    /// * `watching_id` - The ID of the watching to update
    /// * `params` - Parameters for updating the watching
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use backlog_watching::{WatchingApi, UpdateWatchingParams};
    /// # use backlog_core::identifier::WatchingId;
    /// # async fn example(api: WatchingApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = UpdateWatchingParams::new(WatchingId::from(123))
    ///     .with_note("Updated note");
    /// let watching = api.update(params).await?;
    /// println!("Updated watching note: {:?}", watching.note);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "writable")]
    pub async fn update(&self, params: UpdateWatchingParams) -> Result<UpdateWatchingResponse> {
        self.client.execute(params).await
    }

    /// Deletes an existing watching.
    ///
    /// Corresponds to `DELETE /api/v2/watchings/:watchingId`.
    ///
    /// # Arguments
    ///
    /// * `watching_id` - The ID of the watching to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use backlog_watching::WatchingApi;
    /// # use backlog_core::identifier::WatchingId;
    /// # async fn example(api: WatchingApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let deleted_watching = api.delete(WatchingId::from(123)).await?;
    /// println!("Deleted watching for issue: {:?}", deleted_watching.issue);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "writable")]
    pub async fn delete(
        &self,
        watching_id: impl Into<WatchingId>,
    ) -> Result<DeleteWatchingResponse> {
        let params = DeleteWatchingParams::new(watching_id);
        self.client.execute(params).await
    }

    /// Marks a watching as read.
    ///
    /// Corresponds to `POST /api/v2/watchings/:watchingId/markAsRead`.
    ///
    /// # Arguments
    ///
    /// * `watching_id` - The ID of the watching to mark as read
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use backlog_watching::WatchingApi;
    /// # use backlog_core::identifier::WatchingId;
    /// # async fn example(api: WatchingApi) -> Result<(), Box<dyn std::error::Error>> {
    /// api.mark_as_read(WatchingId::from(123)).await?;
    /// println!("Watching marked as read");
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "writable")]
    pub async fn mark_as_read(&self, watching_id: impl Into<WatchingId>) -> Result<()> {
        let params = MarkAsReadParams::new(watching_id);
        self.client.execute_no_content(params).await
    }
}
