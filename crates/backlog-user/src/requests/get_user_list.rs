use backlog_api_core::{HttpMethod, IntoRequest};

/// Parameters for getting the list of users.
///
/// Corresponds to `GET /api/v2/users`.
#[derive(Debug, Clone, Default)]
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
}
