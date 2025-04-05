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
}

type GetUserResponse = User;
