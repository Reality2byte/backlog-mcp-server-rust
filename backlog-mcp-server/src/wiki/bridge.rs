use crate::access_control::AccessControl;
use crate::error::{Error as McpError, Result};
use crate::wiki::request::{
    DownloadWikiAttachmentRequest, GetWikiAttachmentListRequest, GetWikiDetailRequest,
    GetWikiListRequest,
};

#[cfg(feature = "wiki_writable")]
use crate::wiki::request::UpdateWikiRequest;
use backlog_api_client::{
    DownloadedFile, GetWikiListParams, ProjectIdOrKey, client::BacklogApiClient,
};
use backlog_wiki::{
    DownloadWikiAttachmentParams, GetWikiAttachmentListParams, GetWikiDetailParams,
};

use backlog_core::{
    ProjectKey,
    identifier::{ProjectId, WikiAttachmentId, WikiId},
};
#[cfg(feature = "wiki_writable")]
use backlog_wiki::UpdateWikiParams;
use std::str::FromStr;

pub(crate) async fn get_wiki_list(
    client: &BacklogApiClient,
    request: GetWikiListRequest,
    access_control: &AccessControl,
) -> Result<serde_json::Value> {
    let wiki_api = client.wiki();

    let mut params = GetWikiListParams::new();

    // Handle project_id_or_key parameter
    if let Some(project_str) = request.project_id_or_key {
        let project_id_or_key = if let Ok(id) = project_str.parse::<u32>() {
            ProjectIdOrKey::Id(ProjectId::new(id))
        } else if let Ok(key) = ProjectKey::from_str(&project_str) {
            ProjectIdOrKey::Key(key)
        } else {
            return Err(McpError::Parameter(format!(
                "Invalid project ID or key: {project_str}. Must be a numeric ID or valid project key.",
            )));
        };
        params = params.project_id_or_key(project_id_or_key);
    }

    // Handle keyword parameter
    if let Some(keyword) = request.keyword {
        params = params.keyword(keyword);
    }

    let wikis = wiki_api.get_wiki_list(params).await?;

    // Check project access for each wiki in the response
    for wiki in &wikis {
        access_control.check_project_access_by_id(&wiki.project_id)?;
    }

    Ok(serde_json::to_value(wikis)?)
}

pub(crate) async fn get_wiki_detail(
    client: &BacklogApiClient,
    request: GetWikiDetailRequest,
    access_control: &AccessControl,
) -> Result<serde_json::Value> {
    let wiki_api = client.wiki();
    let wiki_id = WikiId::new(request.wiki_id);

    let wiki_detail = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(wiki_id))
        .await?;

    // Check project access from the response
    access_control.check_project_access_by_id(&wiki_detail.project_id)?;

    Ok(serde_json::to_value(wiki_detail)?)
}

pub(crate) async fn get_wiki_attachment_list(
    client: &BacklogApiClient,
    request: GetWikiAttachmentListRequest,
    access_control: &AccessControl,
) -> Result<serde_json::Value> {
    let wiki_api = client.wiki();
    let wiki_id = WikiId::new(request.wiki_id);

    // First get wiki details to check project access
    let wiki_detail = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(wiki_id))
        .await?;

    access_control.check_project_access_by_id(&wiki_detail.project_id)?;

    let attachments = wiki_api
        .get_wiki_attachment_list(GetWikiAttachmentListParams::new(wiki_id))
        .await?;

    Ok(serde_json::to_value(attachments)?)
}

pub(crate) async fn download_wiki_attachment(
    client: &BacklogApiClient,
    request: DownloadWikiAttachmentRequest,
    access_control: &AccessControl,
) -> Result<DownloadedFile> {
    let wiki_api = client.wiki();
    let wiki_id = WikiId::new(request.wiki_id);

    // First get wiki details to check project access
    let wiki_detail = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(wiki_id))
        .await?;

    access_control.check_project_access_by_id(&wiki_detail.project_id)?;

    let attachment_id = WikiAttachmentId::new(request.attachment_id);
    let downloaded_file = wiki_api
        .download_wiki_attachment(DownloadWikiAttachmentParams::new(wiki_id, attachment_id))
        .await?;

    Ok(downloaded_file)
}

#[cfg(feature = "wiki_writable")]
pub(crate) async fn update_wiki(
    client: &BacklogApiClient,
    request: UpdateWikiRequest,
    access_control: &AccessControl,
) -> Result<serde_json::Value> {
    let wiki_api = client.wiki();
    let wiki_id = WikiId::new(request.wiki_id);

    // First get wiki details to check project access
    let wiki_detail_before = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(wiki_id))
        .await?;

    access_control.check_project_access_by_id(&wiki_detail_before.project_id)?;

    // Build UpdateWikiParams from request
    let mut params = UpdateWikiParams::new(wiki_id);

    if let Some(name) = request.name {
        params = params.name(name);
    }

    if let Some(content) = request.content {
        params = params.content(content);
    }

    if let Some(mail_notify) = request.mail_notify {
        params = params.mail_notify(mail_notify);
    }

    let wiki_detail = wiki_api.update_wiki(params).await?;

    Ok(serde_json::to_value(wiki_detail)?)
}
