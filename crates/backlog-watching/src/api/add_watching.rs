use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::IssueIdOrKey;
use serde::Serialize;

use crate::models::Watching;

/// Parameters for the Add Watching API.
///
/// This API adds a new watching for a specific issue.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddWatchingParams {
    /// The ID or key of the issue to watch.
    pub issue_id_or_key: IssueIdOrKey,
    /// Optional note for the watching.
    pub note: Option<String>,
}

#[cfg(feature = "writable")]
impl AddWatchingParams {
    /// Creates a new instance with the specified issue ID or key.
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            note: None,
        }
    }

    /// Sets the note for the watching.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Form data conversion for AddWatchingParams
#[cfg(feature = "writable")]
impl From<&AddWatchingParams> for Vec<(String, String)> {
    fn from(params: &AddWatchingParams) -> Self {
        let mut form = Vec::new();
        form.push((
            "issueIdOrKey".to_string(),
            params.issue_id_or_key.to_string(),
        ));

        if let Some(note) = &params.note {
            form.push(("note".to_string(), note.clone()));
        }

        form
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddWatchingParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/watchings".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let form_data: Vec<(String, String)> = self.into();
        form_data
    }
}

/// Response type for Add Watching API
#[cfg(feature = "writable")]
pub type AddWatchingResponse = Watching;

#[cfg(test)]
#[path = "add_watching_test.rs"]
mod tests;
