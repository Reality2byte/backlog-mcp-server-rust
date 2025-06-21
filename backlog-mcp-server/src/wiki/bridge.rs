use crate::error::{Error as McpError, Result};
use crate::wiki::request::{GetWikiDetailRequest, GetWikiListRequest};
use backlog_api_client::{GetWikiListParamsBuilder, ProjectIdOrKey, client::BacklogApiClient};
use backlog_core::{
    ProjectKey,
    identifier::{ProjectId, WikiId},
};
use std::str::FromStr;

pub(crate) async fn get_wiki_list(
    client: &BacklogApiClient,
    request: GetWikiListRequest,
) -> Result<serde_json::Value> {
    let wiki_api = client.wiki();

    let mut params_builder = GetWikiListParamsBuilder::default();

    // Handle project_id_or_key parameter
    if let Some(project_str) = request.project_id_or_key {
        let project_id_or_key = if let Ok(id) = project_str.parse::<u32>() {
            ProjectIdOrKey::Id(ProjectId::new(id))
        } else if let Ok(key) = ProjectKey::from_str(&project_str) {
            ProjectIdOrKey::Key(key)
        } else {
            return Err(McpError::Parameter(format!(
                "Invalid project ID or key: {}. Must be a numeric ID or valid project key.",
                project_str
            )));
        };
        params_builder.project_id_or_key(project_id_or_key);
    }

    // Handle keyword parameter
    if let Some(keyword) = request.keyword {
        params_builder.keyword(keyword);
    }

    let params = params_builder.build()?;

    let wikis = wiki_api.get_wiki_list(params).await?;

    Ok(serde_json::to_value(wikis)?)
}

pub(crate) async fn get_wiki_detail(
    client: &BacklogApiClient,
    request: GetWikiDetailRequest,
) -> Result<serde_json::Value> {
    let wiki_api = client.wiki();
    let wiki_id = WikiId::new(request.wiki_id);

    let wiki_detail = wiki_api.get_wiki_detail(wiki_id).await?;

    Ok(serde_json::to_value(wiki_detail)?)
}
