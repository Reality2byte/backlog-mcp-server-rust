use backlog_api_core::Result;
use backlog_core::IssueKey;
use client::Client;

use crate::{requests::AddIssueParams, Issue};

pub struct IssueApi(Client);

impl IssueApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_issue(&self, issue_key: IssueKey) -> Result<GetIssueResponse> {
        self.0.get(&format!("/api/v2/issues/{}", issue_key)).await
    }

    #[cfg(feature = "writable")]
    pub async fn add_issue(&self, params: AddIssueParams) -> Result<GetIssueResponse> {
        self.0.post("/api/v2/issues", &params).await
    }
}

type GetIssueResponse = Issue;
