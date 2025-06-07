use backlog_api_core::Result;
use backlog_core::User;
use client::Client;

pub struct UserApi(Client);

impl UserApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_own_user(&self) -> Result<GetUserResponse> {
        self.0.get("/api/v2/users/myself").await
    }

    /// Fetches the list of users in the space.
    /// Corresponds to `GET /api/v2/users`.
    pub async fn get_user_list(&self) -> Result<Vec<User>> {
        self.0.get("/api/v2/users").await
    }
}

type GetUserResponse = User;
// No specific response type needed for get_user_list as it returns Vec<User> directly.
