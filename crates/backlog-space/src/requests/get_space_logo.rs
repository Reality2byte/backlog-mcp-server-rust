use backlog_api_core::{HttpMethod, IntoRequest};

/// Parameters for getting space logo.
///
/// Corresponds to `GET /api/v2/space/image`.
#[derive(Debug, Clone, Default)]
pub struct GetSpaceLogoParams;

impl GetSpaceLogoParams {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl IntoRequest for GetSpaceLogoParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/space/image".to_string()
    }
}
