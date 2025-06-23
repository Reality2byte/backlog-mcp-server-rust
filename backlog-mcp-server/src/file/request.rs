use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetSharedFilesListRequest {
    #[schemars(
        description = "The project ID or project key to retrieve shared files for. Examples: 'MYPROJECTKEY', '123'."
    )]
    pub project_id_or_key: String,
    #[schemars(
        description = "The path to retrieve shared files from. Root directory is '', not '/'."
    )]
    pub path: String,
    #[schemars(description = "Sort order: 'asc' or 'desc'.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[schemars(description = "Offset for pagination.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[schemars(description = "Number of items to retrieve.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct DownloadSharedFileRequest {
    #[schemars(description = "The project ID or project key. Examples: 'MYPROJECTKEY', '123'.")]
    pub project_id_or_key: String,
    #[schemars(description = "The shared file ID to download.")]
    pub shared_file_id: u32,
    #[schemars(
        description = "Optional format specification: 'image', 'text', or 'raw'. If not specified, format will be auto-detected."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}
