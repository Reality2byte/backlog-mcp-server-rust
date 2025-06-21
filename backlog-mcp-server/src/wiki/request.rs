use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetWikiDetailRequest {
    #[schemars(description = "Wiki page ID to retrieve details for. Must be a positive integer.")]
    pub wiki_id: u32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetWikiListRequest {
    #[schemars(
        description = "Optional project ID or project key to filter wiki pages. Examples: \"MYPROJECTKEY\", \"123\"."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id_or_key: Option<String>,
    #[schemars(description = "Optional keyword to search for in wiki page names or content.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct GetWikiAttachmentListRequest {
    #[schemars(
        description = "Wiki page ID to retrieve attachments for. Must be a positive integer."
    )]
    pub wiki_id: u32,
}
