use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::WatchingId;
use serde::Serialize;

use crate::models::Watching;

/// Parameters for the Get Watching API.
///
/// This API retrieves details of a specific watching by its ID.
#[derive(Debug, Clone)]
pub struct GetWatchingParams {
    /// The ID of the watching to retrieve.
    pub watching_id: WatchingId,
}

impl GetWatchingParams {
    /// Creates a new instance with the specified watching ID.
    pub fn new(watching_id: impl Into<WatchingId>) -> Self {
        Self {
            watching_id: watching_id.into(),
        }
    }
}

/// Empty query parameters for serialization
#[derive(Debug, Clone, Serialize, Default)]
#[allow(dead_code)]
struct QueryParams {}

impl IntoRequest for GetWatchingParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/watchings/{}", self.watching_id)
    }

    fn to_query(&self) -> impl Serialize {
        QueryParams::default()
    }
}

/// Response type for Get Watching API
pub type GetWatchingResponse = Watching;

#[cfg(test)]
#[path = "get_watching_test.rs"]
mod tests;
