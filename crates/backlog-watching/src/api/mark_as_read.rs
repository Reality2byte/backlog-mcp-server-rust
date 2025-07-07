use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::WatchingId;
use serde::Serialize;

/// Parameters for the Mark as Read API.
///
/// This API marks a watching as read.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct MarkAsReadParams {
    /// The ID of the watching to mark as read.
    pub watching_id: WatchingId,
}

#[cfg(feature = "writable")]
impl MarkAsReadParams {
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

/// Empty form data
#[derive(Debug, Clone, Serialize, Default)]
#[allow(dead_code)]
struct FormData {}

#[cfg(feature = "writable")]
impl IntoRequest for MarkAsReadParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/watchings/{}/markAsRead", self.watching_id)
    }

    fn to_form(&self) -> impl Serialize {
        let form_data: Vec<(String, String)> = Vec::new();
        form_data
    }
}

#[cfg(test)]
#[path = "mark_as_read_test.rs"]
mod tests;
