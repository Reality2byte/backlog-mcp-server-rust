use crate::models::WikiAttachment;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::identifier::{Identifier, WikiId};

pub type GetWikiAttachmentListResponse = Vec<WikiAttachment>;

#[derive(Debug, Clone)]
pub struct GetWikiAttachmentListParams {
    pub wiki_id: WikiId,
}

impl GetWikiAttachmentListParams {
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
