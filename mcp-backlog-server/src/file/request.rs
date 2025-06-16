use rmcp::schemars;
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetSharedFilesListRequest {
    /// The project ID or project key to retrieve shared files for.
    /// Examples: "MYPROJECTKEY", "123".
    /// Ensure there are no leading or trailing spaces.
    pub project_id_or_key: String,
    /// The directory path to list files from.
    /// Examples: "documents", "images", "".
    /// Use empty string for root directory.
    pub path: String,
    /// Sort order for the files (optional).
    /// Valid values: "asc", "desc". Default: "desc".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Offset for pagination (optional).
    /// Default: 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Number of files to retrieve (optional).
    /// Valid range: 1-100. Default: 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}
