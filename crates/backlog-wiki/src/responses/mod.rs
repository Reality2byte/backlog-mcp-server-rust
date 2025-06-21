use crate::models::{Wiki, WikiCount, WikiDetail};

pub type GetWikiCountResponse = WikiCount;
pub type GetWikiDetailResponse = WikiDetail;
pub type GetWikiListResponse = Vec<Wiki>;
