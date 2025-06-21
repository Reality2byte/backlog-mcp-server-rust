use crate::models::{Wiki, WikiAttachment, WikiCount, WikiDetail};

pub type GetWikiCountResponse = WikiCount;
pub type GetWikiDetailResponse = WikiDetail;
pub type GetWikiListResponse = Vec<Wiki>;
pub type GetWikiAttachmentListResponse = Vec<WikiAttachment>;
