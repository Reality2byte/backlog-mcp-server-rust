use backlog_api_core::IntoRequest;

pub type GetPriorityListResponse = Vec<backlog_domain_models::Priority>;

// GET /api/v2/priorities
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetPriorityListParams;

impl IntoRequest for GetPriorityListParams {
    fn path(&self) -> String {
        "/api/v2/priorities".to_string()
    }
}
