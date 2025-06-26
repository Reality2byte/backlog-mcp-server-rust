use crate::models::WikiDetail;
/// Corresponds to `DELETE /api/v2/wikis/:wikiId`.
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::WikiId;
use serde::Serialize;

/// Type alias for the response of `delete_wiki` API.
pub type DeleteWikiResponse = WikiDetail;

/// Parameters for deleting a wiki page.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteWikiParams {
    pub wiki_id: WikiId,
    pub mail_notify: Option<bool>,
}

#[cfg(feature = "writable")]
impl DeleteWikiParams {
    /// Creates a new instance of `DeleteWikiParams` with required parameters.
    pub fn new(wiki_id: WikiId) -> Self {
        Self {
            wiki_id,
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
impl IntoRequest for DeleteWikiParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/wikis/{}", self.wiki_id)
    }

    fn to_query(&self) -> impl Serialize {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            mail_notify: Option<bool>,
        }

        Query {
            mail_notify: self.mail_notify,
        }
    }
}
