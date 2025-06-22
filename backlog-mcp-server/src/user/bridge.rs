use crate::error::Result;
use crate::user::request::GetUserListRequest;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::User;
use backlog_user::GetUserListParams;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) async fn get_user_list_bridge(
    client: Arc<Mutex<BacklogApiClient>>,
    _req: GetUserListRequest,
) -> Result<Vec<User>> {
    let client_guard = client.lock().await;
    let param = GetUserListParams::new();
    Ok(client_guard.user().get_user_list(param).await?)
}
