use backlog_api_core::{Error as ApiError, HttpMethod, IntoDownloadRequest, IntoRequest};
use backlog_core::identifier::UserId;
use client::DownloadedFile;
use derive_builder::Builder;
use serde::Serialize;

/// Response type for getting user icon
pub type GetUserIconResponse = DownloadedFile;

/// Parameters for getting a user's icon image data.
///
/// Corresponds to `GET /api/v2/users/:userId/icon`.
#[derive(Builder, Debug, Clone, Serialize)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetUserIconParams {
    /// The user ID.
    #[serde(skip)]
    pub user_id: UserId,
}

impl GetUserIconParams {
    /// Creates a new instance with the required parameters.
    pub fn new(user_id: impl Into<UserId>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}

impl IntoRequest for GetUserIconParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/users/{}/icon", self.user_id)
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}

impl IntoDownloadRequest for GetUserIconParams {
    fn path(&self) -> String {
        format!("/api/v2/users/{}/icon", self.user_id)
    }
}
