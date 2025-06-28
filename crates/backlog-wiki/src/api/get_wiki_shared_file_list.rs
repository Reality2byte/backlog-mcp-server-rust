use backlog_api_core::IntoRequest;
use backlog_core::identifier::WikiId;
use backlog_file::models::SharedFile;

/// Response type for getting a list of shared files linked to a wiki page
pub type GetWikiSharedFileListResponse = Vec<SharedFile>;

/// Parameters for getting shared file list for a specific wiki page.
/// Corresponds to `GET /api/v2/wikis/:wikiId/sharedFiles`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetWikiSharedFileListParams {
    pub wiki_id: WikiId,
}

impl GetWikiSharedFileListParams {
    pub fn new(wiki_id: impl Into<WikiId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
        }
    }
}

impl IntoRequest for GetWikiSharedFileListParams {
    fn path(&self) -> String {
        format!("/api/v2/wikis/{}/sharedFiles", self.wiki_id)
    }
}
