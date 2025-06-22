use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::{Identifier, WikiId};
use derive_builder::Builder;

/// Parameters for getting a wiki detail.
///
/// Corresponds to `GET /api/v2/wikis/:wikiId`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "backlog_api_core::Error"))]
pub struct GetWikiDetailParams {
    /// The wiki ID.
    pub wiki_id: WikiId,
}

impl GetWikiDetailParams {
    /// Creates a new instance with the required parameters.
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
        }
    }
}

impl IntoRequest for GetWikiDetailParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/wikis/{}", self.wiki_id.value())
    }
}
