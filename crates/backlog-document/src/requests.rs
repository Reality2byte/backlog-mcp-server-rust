use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use backlog_core::identifier::ProjectId;
use derive_builder::Builder;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Builder, Clone, PartialEq)]
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

#[derive(Debug, Clone, Serialize, PartialEq)]
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

#[derive(Debug, Builder, Clone, PartialEq)]
#[builder(setter(strip_option), build_fn(error = "backlog_api_core::Error"))]
pub struct GetDocumentTreeParams {
    // Based on curl: /api/v2/documents/tree?apiKey=xxx&projectIdOrKey=MSSP
    #[builder(setter(into))]
    pub project_id_or_key: ProjectIdOrKey,
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

/// Parameters for getting a specific document.
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

/// Parameters for downloading document attachment.
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadAttachmentParams {
    pub document_id: backlog_core::identifier::DocumentId,
    pub attachment_id: backlog_core::identifier::DocumentAttachmentId,
}

impl DownloadAttachmentParams {
    pub fn new(
        document_id: impl Into<backlog_core::identifier::DocumentId>,
        attachment_id: impl Into<backlog_core::identifier::DocumentAttachmentId>,
    ) -> Self {
        Self {
            document_id: document_id.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

// IntoRequest implementations for unified access control
impl IntoRequest for ListDocumentsParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/documents".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self.clone())
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

impl IntoRequest for GetDocumentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/documents/{}", self.document_id)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

impl IntoRequest for DownloadAttachmentParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/documents/{}/attachments/{}",
            self.document_id, self.attachment_id
        )
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
