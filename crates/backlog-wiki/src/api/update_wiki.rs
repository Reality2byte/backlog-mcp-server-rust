#[cfg(feature = "writable")]
use crate::models::WikiDetail;
#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::identifier::{Identifier, WikiId};
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateWikiResponse = WikiDetail;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWikiParams {
    #[serde(skip)]
    pub wiki_id: WikiId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_notify: Option<bool>,
}

#[cfg(feature = "writable")]
impl UpdateWikiParams {
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
            name: None,
            content: None,
            mail_notify: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn mail_notify(mut self, mail_notify: bool) -> Self {
        self.mail_notify = Some(mail_notify);
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateWikiParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/wikis/{}", self.wiki_id.value())
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
