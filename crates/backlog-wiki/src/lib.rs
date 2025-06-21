pub mod api;
pub mod models;
pub mod requests;
pub mod responses;

pub use api::WikiApi;
pub use models::{Wiki, WikiAttachment, WikiCount, WikiDetail, WikiTag};
pub use requests::{
    GetWikiCountParams, GetWikiCountParamsBuilder, GetWikiListParams, GetWikiListParamsBuilder,
};
pub use responses::{GetWikiCountResponse, GetWikiDetailResponse, GetWikiListResponse};
