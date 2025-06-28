#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::identifier::{SharedFileId, WikiId};
#[cfg(feature = "writable")]
use backlog_file::models::SharedFile;
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Response type for linking shared files to a wiki page
#[cfg(feature = "writable")]
pub type LinkSharedFilesToWikiResponse = Vec<SharedFile>;

/// Parameters for linking shared files to a wiki page.
/// Corresponds to `POST /api/v2/wikis/:wikiId/sharedFiles`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "writable", derive(ToFormParams))]
#[builder(build_fn(error = "ApiError"))]
pub struct LinkSharedFilesToWikiParams {
    #[builder(setter(into))]
    #[cfg_attr(feature = "writable", form(skip))]
    pub wiki_id: WikiId,
    #[builder(setter(into))]
    #[cfg_attr(feature = "writable", form(array, name = "fileId"))]
    pub shared_file_ids: Vec<SharedFileId>,
}

#[cfg(feature = "writable")]
impl LinkSharedFilesToWikiParams {
    /// Creates new parameters for linking shared files to a wiki page.
    pub fn new(wiki_id: impl Into<WikiId>, shared_file_ids: Vec<SharedFileId>) -> Self {
        Self {
            wiki_id: wiki_id.into(),
            shared_file_ids,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for LinkSharedFilesToWikiParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/wikis/{}/sharedFiles", self.wiki_id)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
