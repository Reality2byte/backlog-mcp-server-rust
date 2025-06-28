#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::identifier::{SharedFileId, WikiId};
#[cfg(feature = "writable")]
use backlog_file::models::SharedFile;

/// Response type for unlinking shared file from wiki page operations.
#[cfg(feature = "writable")]
pub type UnlinkSharedFileFromWikiResponse = SharedFile;

/// Parameters for unlinking a shared file from a wiki page.
/// Corresponds to `DELETE /api/v2/wikis/:wikiId/sharedFiles/:id`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UnlinkSharedFileFromWikiParams {
    pub wiki_id: WikiId,
    pub shared_file_id: SharedFileId,
}

#[cfg(feature = "writable")]
impl UnlinkSharedFileFromWikiParams {
    /// Creates new parameters for unlinking a shared file from a wiki page.
    pub fn new(wiki_id: impl Into<WikiId>, shared_file_id: impl Into<SharedFileId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
            shared_file_id: shared_file_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UnlinkSharedFileFromWikiParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/wikis/{}/sharedFiles/{}",
            self.wiki_id, self.shared_file_id
        )
    }
}
