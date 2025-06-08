use crate::error::Result;
use crate::user::request::GetUserListRequest;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::User;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) async fn get_user_list_bridge(
    _req: GetUserListRequest,
    client: Arc<Mutex<BacklogApiClient>>,
) -> Result<Vec<User>> {
    let client_guard = client.lock().await;
    // The get_user_list method in backlog_user::UserApi takes no parameters.
    Ok(client_guard.user().get_user_list().await?)
}
