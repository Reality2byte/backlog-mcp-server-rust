// Main API struct
mod document_api;
pub use document_api::DocumentApi;

// Read-only API modules
mod download_attachment;
mod get_document;
mod get_document_tree;
mod list_documents;

// Re-export parameter types and response types
pub use download_attachment::DownloadAttachmentParams;
pub use get_document::{GetDocumentParams, GetDocumentResponse};
pub use get_document_tree::{GetDocumentTreeParams, GetDocumentTreeResponse};
pub use list_documents::{
    DocumentOrder, DocumentSortKey, ListDocumentsParams, ListDocumentsParamsBuilder,
    ListDocumentsResponse,
};
