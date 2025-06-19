use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetDocumentDetailsRequest {
    #[schemars(description = "The document id to retrieve details for. 
    This should be in the format 32 digit hex string. Ensure there are no leading or trailing spaces.
    When you access https://example.backlog.com/document/PROJECT/0195faa11fcb7aaab4c4005a7ada4b6f,
    the document id is '0195faa11fcb7aaab4c4005a7ada4b6f'.")]
    pub document_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct DownloadDocumentAttachmentRequest {
    #[schemars(description = "The document ID (a 32-digit hexadecimal string).")]
    pub document_id: String,
    #[schemars(description = "The numeric ID of the attachment to download.")]
    pub attachment_id: u32,
    #[schemars(
        description = "Optional format specification: 'image', 'text', or 'raw'. If not specified, format will be auto-detected."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetDocumentTreeRequest {
    #[schemars(
        description = "The project ID or project key for which to retrieve the document tree. Examples: \"MYPROJECTKEY\", \"123\"."
    )]
    pub project_id_or_key: String,
}
