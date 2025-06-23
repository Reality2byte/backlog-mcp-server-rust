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
#[derive(Debug, Clone)]
pub struct UpdateWikiParams {
    pub wiki_id: WikiId,
    pub name: Option<String>,
    pub content: Option<String>,
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
impl From<&UpdateWikiParams> for Vec<(String, String)> {
    fn from(params: &UpdateWikiParams) -> Self {
        let mut seq = Vec::new();

        if let Some(name) = &params.name {
            seq.push(("name".to_string(), name.clone()));
        }

        if let Some(content) = &params.content {
            seq.push(("content".to_string(), content.clone()));
        }

        if let Some(mail_notify) = params.mail_notify {
            seq.push(("mailNotify".to_string(), mail_notify.to_string()));
        }

        seq
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
        let params: Vec<(String, String)> = self.into();
        params
    }
}
