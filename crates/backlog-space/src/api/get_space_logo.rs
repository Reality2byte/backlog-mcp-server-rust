use backlog_api_core::IntoDownloadRequest;
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

impl IntoDownloadRequest for GetSpaceLogoParams {
    fn path(&self) -> String {
        "/api/v2/space/image".to_string()
    }
}
