use backlog_api_core::{HttpMethod, IntoRequest};
use serde::Serialize;

pub type GetPriorityListResponse = Vec<backlog_domain_models::Priority>;

// GET /api/v2/priorities
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetPriorityListParams;

impl IntoRequest for GetPriorityListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/priorities".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
