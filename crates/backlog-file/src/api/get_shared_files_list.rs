use crate::models::SharedFile;
use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use derive_builder::Builder;
use serde::Serialize;

/// Response type for getting shared files list
pub type GetSharedFilesListResponse = Vec<SharedFile>;

/// Parameters for getting shared files list
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/files/metadata/:path`.
#[derive(Debug, Clone, PartialEq, Builder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct GetSharedFilesListParams {
    /// Project ID or key
    #[builder(setter(into))]
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,

    /// Path to the directory
    #[builder(setter(into))]
    #[serde(skip)]
    pub path: String,

    /// Sort order for the files ("asc" or "desc")
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Offset for pagination
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Number of files to retrieve (1-100, default: 20)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

impl IntoRequest for GetSharedFilesListParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/files/metadata/{}",
            self.project_id_or_key, self.path
        )
    }

    fn to_query(&self) -> impl serde::Serialize {
        self
    }
}
