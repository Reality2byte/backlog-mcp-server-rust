use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::{Identifier, WikiId};
use derive_builder::Builder;

/// Parameters for getting a wiki attachment list.
///
/// Corresponds to `GET /api/v2/wikis/:wikiId/attachments`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "backlog_api_core::Error"))]
pub struct GetWikiAttachmentListParams {
    /// The wiki ID.
    pub wiki_id: WikiId,
}

impl GetWikiAttachmentListParams {
    /// Creates a new instance with the required parameters.
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
        }
    }
}

impl IntoRequest for GetWikiAttachmentListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/wikis/{}/attachments", self.wiki_id.value())
    }
}
