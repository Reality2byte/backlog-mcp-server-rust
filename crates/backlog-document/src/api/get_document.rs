use crate::models::DocumentDetail;
use backlog_api_core::IntoRequest;
use serde::Serialize;

/// Response type for getting a specific document
pub type GetDocumentResponse = DocumentDetail;

/// Parameters for getting a specific document
///
/// Corresponds to `GET /api/v2/documents/:documentId`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetDocumentParams {
    pub document_id: backlog_core::identifier::DocumentId,
}

impl GetDocumentParams {
    pub fn new(document_id: impl Into<backlog_core::identifier::DocumentId>) -> Self {
        Self {
            document_id: document_id.into(),
        }
    }
}

impl IntoRequest for GetDocumentParams {
    fn path(&self) -> String {
        format!("/api/v2/documents/{}", self.document_id)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
