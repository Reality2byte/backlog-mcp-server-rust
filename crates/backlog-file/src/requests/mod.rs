use crate::models::SharedFile;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub type GetSharedFilesListResponse = Vec<SharedFile>;

/// Parameters for getting shared files list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Default)]
#[builder(setter(strip_option))]
pub struct GetSharedFilesListParams {
    /// Sort order for the files ("asc" or "desc")
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub order: Option<String>,
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<u32>,
    /// Number of files to retrieve (1-100, default: 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<u32>,
}

/// Convert parameters to query string format for HTTP client
impl From<GetSharedFilesListParams> for Vec<(String, String)> {
    fn from(params: GetSharedFilesListParams) -> Self {
        let mut query_params = Vec::new();

        if let Some(order) = params.order {
            query_params.push(("order".to_string(), order));
        }
        if let Some(offset) = params.offset {
            query_params.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(count) = params.count {
            query_params.push(("count".to_string(), count.to_string()));
        }

        query_params
    }
}
