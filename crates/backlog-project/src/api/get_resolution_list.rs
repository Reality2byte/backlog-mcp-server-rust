use backlog_api_core::IntoRequest;
use serde::Serialize;

pub type GetResolutionListResponse = Vec<backlog_domain_models::Resolution>;

// GET /api/v2/resolutions
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetResolutionListParams;

impl IntoRequest for GetResolutionListParams {
    fn path(&self) -> String {
        "/api/v2/resolutions".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
