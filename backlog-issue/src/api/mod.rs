use backlog_api_core::Result;
use backlog_core::{IssueIdOrKey, IssueKey};
use client::Client;

use crate::{
    requests::{AddIssueParams, CountIssueParams, UpdateIssueParams},
    responses::CountIssueResponse,
    Issue,
};

pub struct IssueApi(Client);

impl IssueApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_issue(&self, issue_key: IssueKey) -> Result<GetIssueResponse> {
        self.0.get(&format!("/api/v2/issues/{}", issue_key)).await
    }

    pub async fn count_issue(&self, params: CountIssueParams) -> Result<CountIssueResponse> {
        let params: Vec<(String, String)> = params.into();
        self.0
            .get_with_params("/api/v2/issues/count", &params)
            .await
    }

    #[cfg(feature = "writable")]
    pub async fn add_issue(&self, params: AddIssueParams) -> Result<AddIssueResponse> {
        self.0.post("/api/v2/issues", &params).await
    }

    #[cfg(feature = "writable")]
    pub async fn delete_issue(&self, issue_key: IssueKey) -> Result<DeleteIssueResponse> {
        self.0
            .delete(&format!("/api/v2/issues/{}", issue_key))
            .await
    }

    #[cfg(feature = "writable")]
    pub async fn update_issue(
        &self,
        issue_id_or_key: impl Into<IssueIdOrKey>,
        params: &UpdateIssueParams,
    ) -> Result<UpdateIssueResponse> {
        let issue_id_or_key_str: String = issue_id_or_key.into().into();
        self.0
            .patch(&format!("/api/v2/issues/{}", issue_id_or_key_str), params)
            .await
    }
}

type GetIssueResponse = Issue;
type AddIssueResponse = Issue;
type DeleteIssueResponse = Issue;
type UpdateIssueResponse = Issue;
