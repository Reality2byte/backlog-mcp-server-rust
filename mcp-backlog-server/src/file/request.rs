use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetSharedFilesListRequest {
    #[schemars(
        description = "The project ID or project key to retrieve shared files for. Examples: 'MYPROJECTKEY', '123'."
    )]
    pub project_id_or_key: String,
    #[schemars(description = "The path to retrieve shared files from.")]
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
