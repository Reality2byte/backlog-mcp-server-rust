use crate::models::DocumentTreeRootNode;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::{ProjectIdOrKey, identifier::ProjectId};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Response type for getting document tree
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetDocumentTreeResponse {
    pub project_id: ProjectId,
    pub active_tree: DocumentTreeRootNode,
    pub trash_tree: DocumentTreeRootNode,
}

/// Parameters for getting document tree
///
/// Corresponds to `GET /api/v2/documents/tree`.
#[derive(Debug, Builder, Clone, PartialEq)]
#[builder(setter(strip_option), build_fn(error = "backlog_api_core::Error"))]
pub struct GetDocumentTreeParams {
    // Based on curl: /api/v2/documents/tree?apiKey=xxx&projectIdOrKey=MSSP
    #[builder(setter(into))]
    pub project_id_or_key: ProjectIdOrKey,
}

impl From<GetDocumentTreeParams> for Vec<(String, String)> {
    fn from(params: GetDocumentTreeParams) -> Self {
        vec![(
            "projectIdOrKey".to_string(),
            params.project_id_or_key.to_string(),
        )]
    }
}

impl IntoRequest for GetDocumentTreeParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/documents/tree".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self.clone())
    }
}
