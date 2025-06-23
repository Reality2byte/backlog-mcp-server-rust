mod download_wiki_attachment;
mod get_wiki_attachment_list;
mod get_wiki_count;
mod get_wiki_detail;
mod get_wiki_list;
mod wiki_api;

#[cfg(feature = "writable")]
mod update_wiki;

// Export response types (always available)
pub use download_wiki_attachment::DownloadWikiAttachmentParams;
pub use get_wiki_attachment_list::{GetWikiAttachmentListParams, GetWikiAttachmentListResponse};
pub use get_wiki_count::{GetWikiCountParams, GetWikiCountResponse};
pub use get_wiki_detail::{GetWikiDetailParams, GetWikiDetailResponse};
pub use get_wiki_list::{GetWikiListParams, GetWikiListResponse};

// Export writable types with feature gates
#[cfg(feature = "writable")]
pub use update_wiki::{UpdateWikiParams, UpdateWikiResponse};

pub use wiki_api::WikiApi;
