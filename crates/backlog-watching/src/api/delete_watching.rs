use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::WatchingId;
use serde::Serialize;

use crate::models::Watching;

/// Parameters for the Delete Watching API.
///
/// This API deletes an existing watching.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteWatchingParams {
    /// The ID of the watching to delete.
    pub watching_id: WatchingId,
}

#[cfg(feature = "writable")]
impl DeleteWatchingParams {
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

#[cfg(feature = "writable")]
impl IntoRequest for DeleteWatchingParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/watchings/{}", self.watching_id)
    }

    fn to_query(&self) -> impl Serialize {
        QueryParams::default()
    }
}

/// Response type for Delete Watching API
#[cfg(feature = "writable")]
pub type DeleteWatchingResponse = Watching;

#[cfg(test)]
#[path = "delete_watching_test.rs"]
mod tests;
