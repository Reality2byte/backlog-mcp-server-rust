use backlog_api_core::Result;
use backlog_core::{User, identifier::UserId};
use client::Client;

pub struct UserApi(Client);

impl UserApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get the details of the authenticated user.
    pub async fn get_own_user(&self) -> Result<GetUserResponse> {
        self.0.get("/api/v2/users/myself").await
    }

    /// Get the list of users in the space.
    /// Corresponds to `GET /api/v2/users`.
    pub async fn get_user_list(&self) -> Result<Vec<User>> {
        self.0.get("/api/v2/users").await
    }

    /// Gets the user icon image data.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/icon`.
    pub async fn get_user_icon(&self, user_id: impl Into<UserId>) -> Result<Vec<u8>> {
        let path = format!("/api/v2/users/{}/icon", user_id.into());
        let downloaded_file = self.0.download_file_raw(&path).await?;
        Ok(downloaded_file.bytes.to_vec())
    }
}

type GetUserResponse = User;
// No specific response type needed for get_user_list as it returns Vec<User> directly.
