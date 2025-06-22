use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::{Identifier, WikiId};
use derive_builder::Builder;
use serde::Serialize;

/// Parameters for updating a wiki page.
///
/// Corresponds to `PATCH /api/v2/wikis/:wikiId`.
#[cfg(feature = "writable")]
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "backlog_api_core::Error"), setter(strip_option))]
pub struct UpdateWikiRequestParams {
    /// The wiki ID.
    pub wiki_id: WikiId,
    /// New page name (optional).
    #[builder(default, setter(into))]
    pub name: Option<String>,
    /// New page content (optional).
    #[builder(default, setter(into))]
    pub content: Option<String>,
    /// Whether to send email notification of update (optional).
    #[builder(default)]
    pub mail_notify: Option<bool>,
}

#[cfg(feature = "writable")]
impl UpdateWikiRequestParams {
    /// Creates a new instance with the required parameters.
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
            name: None,
            content: None,
            mail_notify: None,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateWikiRequestParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/wikis/{}", self.wiki_id.value())
    }

    fn to_form(&self) -> impl Serialize {
        let mut seq = Vec::new();

        if let Some(name) = &self.name {
            seq.push(("name".to_string(), name.clone()));
        }

        if let Some(content) = &self.content {
            seq.push(("content".to_string(), content.clone()));
        }

        if let Some(mail_notify) = self.mail_notify {
            seq.push(("mailNotify".to_string(), mail_notify.to_string()));
        }

        seq
    }
}
