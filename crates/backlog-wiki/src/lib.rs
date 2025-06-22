pub mod api;
pub mod models;
pub mod requests;
pub mod responses;

pub use api::WikiApi;
pub use models::{Wiki, WikiAttachment, WikiCount, WikiDetail, WikiTag};
pub use requests::{
    DownloadWikiAttachmentParams, DownloadWikiAttachmentParamsBuilder, GetWikiAttachmentListParams,
    GetWikiAttachmentListParamsBuilder, GetWikiCountParams, GetWikiCountParamsBuilder,
    GetWikiDetailParams, GetWikiDetailParamsBuilder, GetWikiListParams, GetWikiListParamsBuilder,
};

#[cfg(feature = "writable")]
pub use requests::{UpdateWikiParams, UpdateWikiRequestParams, UpdateWikiRequestParamsBuilder};
pub use responses::{
    GetWikiAttachmentListResponse, GetWikiCountResponse, GetWikiDetailResponse, GetWikiListResponse,
};
