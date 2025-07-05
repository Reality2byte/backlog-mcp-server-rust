use backlog_api_core::Result;
use client::Client;

use crate::api::{
    GetOwnUserParams, GetOwnUserResponse, GetUserIconParams, GetUserIconResponse,
    GetUserListParams, GetUserListResponse, GetUserParams, GetUserRecentUpdatesParams,
    GetUserRecentUpdatesResponse, GetUserResponse, GetUserStarCountParams,
    GetUserStarCountResponse, GetUserStarsParams, GetUserStarsResponse,
};

pub struct UserApi(Client);

impl UserApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get the list of users in the space using IntoRequest pattern.
    /// Corresponds to `GET /api/v2/users`.
    pub async fn get_user_list(&self, params: GetUserListParams) -> Result<GetUserListResponse> {
        self.0.execute(params).await
    }

    /// Gets information about a specific user using IntoRequest pattern.
    ///
    /// Corresponds to `GET /api/v2/users/:userId`.
    pub async fn get_user(&self, params: GetUserParams) -> Result<GetUserResponse> {
        self.0.execute(params).await
    }

    /// Get the details of the authenticated user using IntoRequest pattern.
    pub async fn get_own_user(&self, params: GetOwnUserParams) -> Result<GetOwnUserResponse> {
        self.0.execute(params).await
    }

    /// Gets the user icon image data using IntoDownloadRequest pattern.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/icon`.
    pub async fn get_user_icon(&self, params: GetUserIconParams) -> Result<GetUserIconResponse> {
        self.0.download_file(params).await
    }

    /// Gets recent activities for a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/activities`.
    pub async fn get_user_recent_updates(
        &self,
        params: GetUserRecentUpdatesParams,
    ) -> Result<GetUserRecentUpdatesResponse> {
        self.0.execute(params).await
    }

    /// Gets the count of stars received by a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/stars/count`.
    pub async fn get_user_star_count(
        &self,
        params: GetUserStarCountParams,
    ) -> Result<GetUserStarCountResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of stars received by a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/stars`.
    pub async fn get_user_stars(&self, params: GetUserStarsParams) -> Result<GetUserStarsResponse> {
        self.0.execute(params).await
    }
}
