use crate::models::WikiAttachment;
/// Corresponds to `POST /api/v2/wikis/:wikiId/attachments`.
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::{AttachmentId, WikiId};
use serde::Serialize;

/// Type alias for the response of `attach_files_to_wiki` API.
pub type AttachFilesToWikiResponse = Vec<WikiAttachment>;

/// Parameters for attaching files to a wiki page.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct AttachFilesToWikiParams {
    #[form(skip)]
    pub wiki_id: WikiId,
    #[form(array, name = "attachmentId")]
    pub attachment_ids: Vec<AttachmentId>,
}

#[cfg(feature = "writable")]
impl AttachFilesToWikiParams {
    /// Creates a new instance of `AttachFilesToWikiParams` with required parameters.
    pub fn new(wiki_id: WikiId, attachment_ids: Vec<AttachmentId>) -> Self {
        Self {
            wiki_id,
            attachment_ids,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AttachFilesToWikiParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/wikis/{}/attachments", self.wiki_id)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
