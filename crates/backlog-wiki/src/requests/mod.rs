mod get_wiki_count;
mod get_wiki_list;

#[cfg(feature = "writable")]
mod update_wiki;

pub use get_wiki_count::{
    GetWikiCountParams, GetWikiCountParamsBuilder, GetWikiCountParamsBuilderError,
};
pub use get_wiki_list::{
    GetWikiListParams, GetWikiListParamsBuilder, GetWikiListParamsBuilderError,
};

#[cfg(feature = "writable")]
pub use update_wiki::UpdateWikiParams;
