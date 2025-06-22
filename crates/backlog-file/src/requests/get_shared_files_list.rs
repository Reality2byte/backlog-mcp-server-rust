use crate::models::SharedFile;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use derive_builder::Builder;

pub type GetSharedFilesListResponse = Vec<SharedFile>;

/// Parameters for getting shared files list
#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(strip_option))]
pub struct GetSharedFilesListParams {
    /// Project ID or key
    #[builder(setter(into))]
    pub project_id_or_key: ProjectIdOrKey,

    /// Path to the directory
    #[builder(setter(into))]
    pub path: String,

    /// Sort order for the files ("asc" or "desc")
    #[builder(default, setter(into))]
    pub order: Option<String>,

    /// Offset for pagination
    #[builder(default)]
    pub offset: Option<u32>,

    /// Number of files to retrieve (1-100, default: 20)
    #[builder(default)]
    pub count: Option<u32>,
}

impl IntoRequest for GetSharedFilesListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/files/metadata/{}",
            self.project_id_or_key, self.path
        )
    }

    fn to_query(&self) -> impl serde::Serialize {
        let mut query_params = Vec::new();

        if let Some(order) = &self.order {
            query_params.push(("order".to_string(), order.clone()));
        }
        if let Some(offset) = self.offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(count) = self.count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        query_params
    }
}
