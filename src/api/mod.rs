use crate::client::Client;
use crate::error::Error;
use crate::responses::get_recent_updates_response::GetRecentUpdatesResponse;
use crate::responses::get_space_response::GetSpaceResponse;
use crate::types::User;

pub async fn get_space(client: &Client) -> Result<GetSpaceResponse, Error> {
    client.get("/api/v2/space").await
}

pub async fn get_recent_updates(client: &Client) -> Result<GetRecentUpdatesResponse, Error> {
    client.get("/api/v2/space/activities").await
}

type GetUserResponse = User;
pub async fn get_own_user(client: &Client) -> Result<GetUserResponse, Error> {
    client.get("/api/v2/users/myself").await
}
