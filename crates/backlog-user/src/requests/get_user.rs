use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
use backlog_core::identifier::UserId;
use derive_builder::Builder;

/// Parameters for getting a specific user.
///
/// Corresponds to `GET /api/v2/users/:userId`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetUserParams {
    /// The user ID.
    pub user_id: UserId,
}

impl GetUserParams {
    /// Creates a new instance with the required parameters.
    pub fn new(user_id: impl Into<UserId>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}

impl IntoRequest for GetUserParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/users/{}", self.user_id)
    }
}
