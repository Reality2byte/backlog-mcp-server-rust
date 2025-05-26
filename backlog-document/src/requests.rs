use backlog_core::ProjectIdOrKey; // Corrected import
use backlog_core::identifier::ProjectId;
use derive_builder::Builder;
use serde::Serialize; // Only needed if these params are sent as body, for GET they are query params. Builder is enough.
use std::fmt;

#[derive(Debug, Builder, Clone, PartialEq)] // Removed Default
#[builder(setter(strip_option))]
pub struct ListDocumentsParams {
    // Based on curl: /api/v2/documents?apiKey=xxx&projectId=601486&offset=0&count=1
    // Based on OpenAPI: /api/v2/:projectKey/list?keyword=X&sort=Y
    // User confirmed routing definition /api/v2/documents is primary.
    // So, projectId is a query param.
    pub project_id: ProjectId, // Assuming this is mandatory based on curl
    #[builder(default, setter(into))]
    pub keyword: Option<String>,
    #[builder(default, setter(into))]
    pub sort: Option<DocumentSortKey>, // Enum to be defined
    #[builder(default)]
    pub offset: u32,
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, PartialEq)] // Serialize might not be needed if only for query
pub enum DocumentSortKey {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "date")]
    Date,
}

impl fmt::Display for DocumentSortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentSortKey::Name => write!(f, "name"),
            DocumentSortKey::Date => write!(f, "date"),
        }
    }
}

#[derive(Debug, Builder, Clone, PartialEq)] // Removed Default
#[builder(setter(strip_option))]
pub struct GetDocumentTreeParams {
    // Based on curl: /api/v2/documents/tree?apiKey=xxx&projectIdOrKey=MSSP
    #[builder(setter(into))]
    pub project_id_or_key: ProjectIdOrKey, // Assuming ProjectIdOrKey from backlog-core
}

// This From implementation is crucial for client.get_with_params
impl From<ListDocumentsParams> for Vec<(String, String)> {
    fn from(params: ListDocumentsParams) -> Self {
        let mut query_params = Vec::new();
        query_params.push(("projectId".to_string(), params.project_id.to_string()));

        if let Some(keyword) = params.keyword {
            query_params.push(("keyword".to_string(), keyword));
        }
        if let Some(sort) = params.sort {
            query_params.push(("sort".to_string(), sort.to_string()));
        }
        query_params.push(("offset".to_string(), params.offset.to_string()));
        if let Some(count) = params.count {
            query_params.push(("count".to_string(), count.to_string()));
        }
        query_params
    }
}

impl From<GetDocumentTreeParams> for Vec<(String, String)> {
    fn from(params: GetDocumentTreeParams) -> Self {
        vec![(
            "projectIdOrKey".to_string(),
            params.project_id_or_key.to_string(),
        )]
    }
}
