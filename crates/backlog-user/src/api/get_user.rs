use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
use backlog_core::{User, identifier::UserId};
use derive_builder::Builder;
use serde::Serialize;

/// Response type for getting a specific user
pub type GetUserResponse = User;

/// Parameters for getting a specific user.
///
/// Corresponds to `GET /api/v2/users/:userId`.
#[derive(Builder, Debug, Clone, Serialize)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetUserParams {
    /// The user ID.
    #[serde(skip)]
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

    fn to_query(&self) -> impl Serialize {
        self
    }
}
