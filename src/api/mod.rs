use crate::client::Client;
use crate::error::Error;
use crate::responses::get_space_response::GetSpaceResponse;

pub async fn get_space(client: &Client) -> Result<GetSpaceResponse, Error> {
    client.get("/api/v2/space").await
}

#[cfg(test)]
mod test {}
