use backlog_api_core::Result;
use backlog_core::IssueKey;
use client::Client;

use crate::Issue;

pub struct IssueApi(Client);

impl IssueApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_issue(&self, issue_key: IssueKey) -> Result<GetIssueResponse> {
        self.0.get(&format!("/api/v2/issues/{}", issue_key)).await
    }
}

type GetIssueResponse = Issue;
