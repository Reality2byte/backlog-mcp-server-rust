use crate::models::WikiDetail;
/// Corresponds to `POST /api/v2/wikis`.
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::ProjectId;
use serde::Serialize;

/// Type alias for the response of `add_wiki` API.
pub type AddWikiResponse = WikiDetail;

/// Parameters for adding a new wiki page.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct AddWikiParams {
    #[form(skip)]
    pub project_id: ProjectId,
    pub name: String,
    pub content: String,
    #[form(name = "mailNotify")]
    pub mail_notify: Option<bool>,
}

#[cfg(feature = "writable")]
impl AddWikiParams {
    /// Creates a new instance of `AddWikiParams` with required parameters.
    pub fn new(project_id: ProjectId, name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            project_id,
            name: name.into(),
            content: content.into(),
            mail_notify: None,
        }
    }

    /// Sets the mail notification flag.
    pub fn mail_notify(mut self, mail_notify: bool) -> Self {
        self.mail_notify = Some(mail_notify);
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddWikiParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/wikis".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let mut params: Vec<(String, String)> = self.into();
        params.push(("projectId".to_string(), self.project_id.to_string()));
        params
    }
}
