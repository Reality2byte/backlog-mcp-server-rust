use backlog_api_core::Result;
use backlog_core::{IssueIdOrKey, IssueKey};
use client::Client;

use crate::{
    Issue,
    requests::{AddIssueParams, CountIssueParams, GetIssueListParams, UpdateIssueParams},
    responses::CountIssueResponse,
};

pub struct IssueApi(Client);

impl IssueApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_issue(&self, issue_key: impl Into<IssueKey>) -> Result<GetIssueResponse> {
        self.0
            .get(&format!("/api/v2/issues/{}", issue_key.into()))
            .await
    }

    pub async fn get_issue_list(&self, params: GetIssueListParams) -> Result<GetIssueListResponse> {
        let params_vec: Vec<(String, String)> = params.into();
        self.0.get_with_params("/api/v2/issues", &params_vec).await
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
    pub async fn delete_issue(
        &self,
        issue_key: impl Into<IssueKey>,
    ) -> Result<DeleteIssueResponse> {
        self.0
            .delete(&format!("/api/v2/issues/{}", issue_key.into()))
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
type GetIssueListResponse = Vec<Issue>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::GetIssueListParamsBuilder;
    use backlog_core::identifier::ProjectId;
    use client::Client;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_client(mock_server: &MockServer) -> Client {
        Client::new(&mock_server.uri()).expect("Failed to create client")
    }

    #[tokio::test]
    async fn test_get_issue_list_empty_params_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        let expected_issues: Vec<Issue> = vec![
            serde_json::from_value(json!({
                "id": 1, "projectId": 1, "issueKey": "BLG-1", "keyId": 1, "summary": "Test Issue 1",
                "description": "This is a test issue (description)",
                "issueType": {"id": 1, "projectId":1, "name": "Bug", "color": "#ff0000", "displayOrder": 0},
                "priority": {"id": 2, "name": "High"},
                "category": [],
                "versions": [],
                "milestone": [],
                "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
                "created": "2024-03-14T06:35:39Z",
                "updated": "2024-04-13T06:35:39Z",
                "status": {"id": 1, "name": "Open"}
            })).unwrap(),
            serde_json::from_value(json!({
                "id": 2, "projectId": 1, "issueKey": "BLG-2", "keyId": 2, "summary": "Test Issue 2",
                "description": "This is another test issue (description)",
                "issueType": {"id": 2, "projectId":1, "name": "Task", "color": "#00ff00", "displayOrder": 1},
                "priority": {"id": 3, "name": "Normal"},
                "category": [],
                "versions": [],
                "milestone": [],
                "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
                "created": "2024-03-14T06:35:39Z",
                "updated": "2024-04-13T06:35:39Z",
                "status": {"id": 2, "name": "In Progress"}
            })).unwrap(),
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/issues"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issues))
            .mount(&mock_server)
            .await;

        let params = GetIssueListParamsBuilder::default().build().unwrap();

        // Act
        let result = issue_api.get_issue_list(params).await;

        // Assert
        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].id, expected_issues[0].id);
        assert_eq!(issues[1].summary, expected_issues[1].summary);
    }

    #[tokio::test]
    async fn test_get_issue_list_with_project_id_param() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_issues: Vec<Issue> = vec![
             serde_json::from_value(json!({
                "id": 3, "projectId": 123, "issueKey": "XYZ-3", "keyId": 3, "summary": "Filtered Issue",
                "issueType": {"id": 1, "projectId":123, "name": "Bug", "color": "#ff0000", "displayOrder": 0},
                "priority": {"id": 2, "name": "High"},
                "description": "This is another test issue (description)",
                "category": [],
                "versions": [],
                "milestone": [],
                "createdUser": {"id": 1, "userId": "john", "name": "John Doe", "roleType": 1, "mailAddress": "john@example.com", "lastLoginTime": "2025-04-01T06:35:39Z"},
                "created": "2024-03-14T06:35:39Z",
                "updated": "2024-04-13T06:35:39Z",
                "status": {"id": 1, "name": "Open"}
            })).unwrap(),
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/issues"))
            .and(query_param("projectId[]", project_id.to_string()))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issues))
            .mount(&mock_server)
            .await;

        let params = GetIssueListParamsBuilder::default()
            .project_id(vec![project_id.clone()])
            .build()
            .unwrap();

        // Act
        let result = issue_api.get_issue_list(params).await;

        // Assert
        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].project_id, project_id);
    }

    #[tokio::test]
    async fn test_get_issue_list_server_error() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);

        Mock::given(method("GET"))
            .and(path("/api/v2/issues"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = GetIssueListParamsBuilder::default().build().unwrap();

        // Act
        let result = issue_api.get_issue_list(params).await;

        // Assert
        assert!(result.is_err());
        // Further error type inspection could be done here
    }
}
