use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::User;
use serde::Serialize;

/// Response type for getting the authenticated user
pub type GetOwnUserResponse = User;

/// Parameters for getting the authenticated user's details.
///
/// Corresponds to `GET /api/v2/users/myself`.
#[derive(Debug, Clone, Default, Serialize)]
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

    fn to_query(&self) -> impl Serialize {
        self
    }
}
