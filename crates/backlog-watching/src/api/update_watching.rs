use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::WatchingId;
use serde::Serialize;

use crate::models::Watching;

/// Parameters for the Update Watching API.
///
/// This API updates the note of an existing watching.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateWatchingParams {
    /// The ID of the watching to update.
    pub watching_id: WatchingId,
    /// The note to set for the watching.
    pub note: Option<String>,
}

#[cfg(feature = "writable")]
impl UpdateWatchingParams {
    /// Creates a new instance with the specified watching ID.
    pub fn new(watching_id: impl Into<WatchingId>) -> Self {
        Self {
            watching_id: watching_id.into(),
            note: None,
        }
    }

    /// Sets the note for the watching.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Form data conversion for UpdateWatchingParams
#[cfg(feature = "writable")]
impl From<&UpdateWatchingParams> for Vec<(String, String)> {
    fn from(params: &UpdateWatchingParams) -> Self {
        let mut form = Vec::new();

        if let Some(note) = &params.note {
            form.push(("note".to_string(), note.clone()));
        }

        form
    }
}

/// Empty query parameters for serialization
#[derive(Debug, Clone, Serialize, Default)]
#[allow(dead_code)]
struct QueryParams {}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateWatchingParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/watchings/{}", self.watching_id)
    }

    fn to_form(&self) -> impl Serialize {
        let form_data: Vec<(String, String)> = self.into();
        form_data
    }
}

/// Response type for Update Watching API
#[cfg(feature = "writable")]
pub type UpdateWatchingResponse = Watching;

#[cfg(test)]
#[path = "update_watching_test.rs"]
mod tests;
