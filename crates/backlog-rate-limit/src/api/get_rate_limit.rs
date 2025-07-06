use backlog_api_core::{HttpMethod, IntoRequest};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRateLimitParams;

impl GetRateLimitParams {
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetRateLimitParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/rateLimit".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
