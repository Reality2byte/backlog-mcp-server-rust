mod get_wiki_count;
mod get_wiki_list;

pub use get_wiki_count::{
    GetWikiCountParams, GetWikiCountParamsBuilder, GetWikiCountParamsBuilderError,
};
pub use get_wiki_list::{
    GetWikiListParams, GetWikiListParamsBuilder, GetWikiListParamsBuilderError,
};
