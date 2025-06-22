use backlog_api_core::{HttpMethod, IntoRequest};

/// Parameters for getting the authenticated user's details.
///
/// Corresponds to `GET /api/v2/users/myself`.
#[derive(Debug, Clone, Default)]
pub struct GetOwnUserParams;

impl GetOwnUserParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetOwnUserParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/users/myself".to_string()
    }
}
