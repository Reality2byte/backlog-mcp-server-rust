use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::User;
use serde::Serialize;

/// Response type for getting user list
pub type GetUserListResponse = Vec<User>;

/// Parameters for getting the list of users.
///
/// Corresponds to `GET /api/v2/users`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserListParams;

impl GetUserListParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetUserListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/users".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
