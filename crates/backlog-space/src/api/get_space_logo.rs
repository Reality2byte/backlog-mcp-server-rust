use backlog_api_core::{HttpMethod, IntoDownloadRequest, IntoRequest};
use client::DownloadedFile;
use serde::Serialize;

/// Response type for getting space logo
pub type GetSpaceLogoResponse = DownloadedFile;

/// Parameters for getting space logo.
///
/// Corresponds to `GET /api/v2/space/image`.
#[derive(Debug, Clone, Default, Serialize)]
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

    fn to_query(&self) -> impl Serialize {
        self
    }
}

impl IntoDownloadRequest for GetSpaceLogoParams {
    fn path(&self) -> String {
        "/api/v2/space/image".to_string()
    }
}
