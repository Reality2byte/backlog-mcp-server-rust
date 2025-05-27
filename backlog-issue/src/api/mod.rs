use backlog_api_core::Result;
use backlog_core::{IssueIdOrKey, IssueKey, ProjectIdOrKey};
use client::Client;

use crate::{
    Issue, Milestone,
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

    pub async fn get_version_milestone_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<GetVersionMilestoneListResponse> {
        let project_id_or_key_val = project_id_or_key.into();
        let project_id_or_key_str: String = project_id_or_key_val.to_string();
        self.0
            .get(&format!(
                "/api/v2/projects/{}/versions",
                project_id_or_key_str
            ))
            .await
    }
}

type GetIssueResponse = Issue;
type AddIssueResponse = Issue;
type DeleteIssueResponse = Issue;
type UpdateIssueResponse = Issue;
type GetIssueListResponse = Vec<Issue>;
type GetVersionMilestoneListResponse = Vec<Milestone>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::issue::Milestone, requests::GetIssueListParamsBuilder};
    use backlog_core::identifier::{MilestoneId, ProjectId};
    use chrono::TimeZone;
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

    #[tokio::test]
    async fn test_get_version_milestone_list_success() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let project_id_or_key_str = "TEST_PROJECT";
        let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();
        let project_id_numeric = ProjectId::new(1); // Assuming project ID 1 for mock

        let expected_versions: Vec<Milestone> = vec![
            Milestone {
                id: MilestoneId::new(1),
                project_id: project_id_numeric.clone(),
                name: "Version 1.0".to_string(),
                description: Some("Initial release".to_string()),
                start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
                release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
                archived: false,
                display_order: Some(1),
            },
            Milestone {
                id: MilestoneId::new(2),
                project_id: project_id_numeric.clone(),
                name: "Version 1.1".to_string(),
                description: None,
                start_date: None,
                release_due_date: None,
                archived: true,
                display_order: Some(2),
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/versions",
                project_id_or_key.clone().to_string()
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_versions))
            .mount(&mock_server)
            .await;

        // Act
        let result = issue_api
            .get_version_milestone_list(project_id_or_key.clone())
            .await;

        // Assert
        assert!(result.is_ok());
        let versions = result.unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].name, "Version 1.0");
        assert_eq!(versions[1].archived, true);
        assert_eq!(versions[0].display_order, Some(1));
    }

    #[tokio::test]
    async fn test_get_version_milestone_list_error() {
        // Arrange
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let issue_api = IssueApi::new(client);
        let project_id_or_key_str = "TEST_PROJECT_ERROR";
        let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/versions",
                project_id_or_key.clone().to_string()
            )))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        // Act
        let result = issue_api
            .get_version_milestone_list(project_id_or_key.clone())
            .await;

        // Assert
        assert!(result.is_err());
    }
}
