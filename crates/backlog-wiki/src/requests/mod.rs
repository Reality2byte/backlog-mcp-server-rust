mod download_wiki_attachment;
mod get_wiki_attachment_list;
mod get_wiki_count;
mod get_wiki_detail;
mod get_wiki_list;

#[cfg(feature = "writable")]
mod update_wiki;

#[cfg(feature = "writable")]
mod update_wiki_params;

pub use download_wiki_attachment::{
    DownloadWikiAttachmentParams, DownloadWikiAttachmentParamsBuilder,
};
pub use get_wiki_attachment_list::{
    GetWikiAttachmentListParams, GetWikiAttachmentListParamsBuilder,
};
pub use get_wiki_count::{GetWikiCountParams, GetWikiCountParamsBuilder};
pub use get_wiki_detail::{GetWikiDetailParams, GetWikiDetailParamsBuilder};
pub use get_wiki_list::{GetWikiListParams, GetWikiListParamsBuilder};

#[cfg(feature = "writable")]
pub use update_wiki::UpdateWikiParams;

#[cfg(feature = "writable")]
pub use update_wiki_params::{UpdateWikiRequestParams, UpdateWikiRequestParamsBuilder};
