use crate::models::WikiDetail;
use backlog_api_core::IntoRequest;
use backlog_core::identifier::{Identifier, WikiId};

pub type GetWikiDetailResponse = WikiDetail;

#[derive(Debug, Clone)]
pub struct GetWikiDetailParams {
    pub wiki_id: WikiId,
}

impl GetWikiDetailParams {
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
        }
    }
}

impl IntoRequest for GetWikiDetailParams {
    fn path(&self) -> String {
        format!("/api/v2/wikis/{}", self.wiki_id.value())
    }
}
