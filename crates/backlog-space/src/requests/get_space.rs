use backlog_api_core::{HttpMethod, IntoRequest};

/// Parameters for getting space information.
///
/// Corresponds to `GET /api/v2/space`.
#[derive(Debug, Clone, Default)]
pub struct GetSpaceParams;

impl GetSpaceParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetSpaceParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/space".to_string()
    }
}
