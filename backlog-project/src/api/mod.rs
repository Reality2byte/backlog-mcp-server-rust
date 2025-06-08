use backlog_api_core::Result;
use backlog_core::ProjectIdOrKey;
use client::Client;

use crate::models::{IssueType, Milestone, Status};
use crate::requests::{GetProjectListResponse, GetProjectParams, GetProjectResponse};

pub struct ProjectApi(Client);

impl ProjectApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Gets the list of projects.
    /// Corresponds to `GET /api/v2/projects`.
    pub async fn get_project_list(
        &self,
        params: GetProjectParams,
    ) -> Result<GetProjectListResponse> {
        self.0.get_with_params("/api/v2/projects", &params).await
    }

    /// Gets a project by its ID or key.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey`.
    pub async fn get_project(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<GetProjectResponse> {
        self.0
            .get(&format!("/api/v2/projects/{}", project_id_or_key.into()))
            .await
    }

    /// Gets the list of statuses for a project.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/statuses`.
    pub async fn get_status_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<Vec<Status>> {
        let path = format!("/api/v2/projects/{}/statuses", project_id_or_key.into());
        self.0.get(&path).await
    }

    /// Gets the list of issue types for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/issueTypes`.
    pub async fn get_issue_type_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<Vec<IssueType>> {
        let path = format!("/api/v2/projects/{}/issueTypes", project_id_or_key.into());
        self.0.get(&path).await
    }

    /// Gets the list of version milestones for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/versions`.
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

type GetVersionMilestoneListResponse = Vec<Milestone>;

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_api_core::Error as ApiError;
    use backlog_core::identifier::{IssueTypeId, MilestoneId, ProjectId, StatusId}; // Added IssueTypeId
    use chrono::TimeZone;
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_version_milestone_list_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = ProjectApi::new(client);
        let project_id_or_key_str = "TEST_PROJECT";
        let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();
        let project_id_numeric = ProjectId::new(1);

        let expected_versions: Vec<Milestone> = vec![
            Milestone {
                id: MilestoneId::new(1),
                project_id: project_id_numeric,
                name: "Version 1.0".to_string(),
                description: Some("Initial release".to_string()),
                start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
                release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
                archived: false,
                display_order: Some(1),
            },
            Milestone {
                id: MilestoneId::new(2),
                project_id: project_id_numeric,
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
                project_id_or_key.clone()
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_versions))
            .mount(&mock_server)
            .await;
        let result = issue_api
            .get_version_milestone_list(project_id_or_key.clone())
            .await;
        assert!(result.is_ok());
        let versions = result.unwrap();
        assert_eq!(versions.len(), 2);
    }

    #[tokio::test]
    async fn test_get_version_milestone_list_error() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await; // Now uses common setup_client
        let issue_api = ProjectApi::new(client);
        let project_id_or_key_str = "TEST_PROJECT_ERROR";
        let project_id_or_key: ProjectIdOrKey = project_id_or_key_str.parse().unwrap();

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/versions",
                project_id_or_key.clone()
            )))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;
        let result = issue_api
            .get_version_milestone_list(project_id_or_key.clone())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_status_list_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_statuses = vec![
            Status {
                id: StatusId::new(1),
                project_id,
                name: "Open".to_string(),
                color: "#ff0000".to_string(),
                display_order: 1,
            },
            Status {
                id: StatusId::new(2),
                project_id,
                name: "Closed".to_string(),
                color: "#00ff00".to_string(),
                display_order: 2,
            },
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/projects/123/statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_statuses))
            .mount(&server)
            .await;

        let result = project_api.get_status_list(project_id).await;
        assert!(result.is_ok());
        let statuses = result.unwrap();
        assert_eq!(statuses.len(), 2);
        assert_eq!(statuses[0].name, "Open");
        assert_eq!(statuses[1].name, "Closed");
    }

    #[tokio::test]
    async fn test_get_status_list_empty() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let expected_statuses: Vec<Status> = Vec::new();

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/projects/{}/statuses", project_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_statuses))
            .mount(&server)
            .await;

        let result = project_api
            .get_status_list(ProjectIdOrKey::from_str(project_key).unwrap())
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_status_list_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = 999; // Non-existent project

        // Example error response from Backlog API
        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such project.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/projects/{}/statuses", project_id)))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let result = project_api
            .get_status_list(ProjectId::new(project_id))
            .await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such project.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_get_issue_type_list_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id_val = 1;
        let project_id = ProjectId::new(project_id_val);

        let expected_issue_types = vec![
            IssueType {
                id: IssueTypeId::new(1),
                project_id,
                name: "バグ".to_string(),
                color: "#990000".to_string(),
                display_order: 0,
                template_summary: Some("件名".to_string()),
                template_description: Some("詳細".to_string()),
            },
            IssueType {
                id: IssueTypeId::new(2),
                project_id,
                name: "タスク".to_string(),
                color: "#009900".to_string(),
                display_order: 1,
                template_summary: None,
                template_description: None,
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/issueTypes",
                project_id_val
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_types))
            .mount(&server)
            .await;

        let result = project_api.get_issue_type_list(project_id).await;
        assert!(result.is_ok(), "Result was: {:?}", result);
        let issue_types = result.unwrap();
        assert_eq!(issue_types.len(), 2);
        assert_eq!(issue_types[0].name, "バグ");
        assert_eq!(issue_types[1].color, "#009900");
    }

    #[tokio::test]
    async fn test_get_issue_type_list_empty() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "EMPTY_PROJECT";

        let expected_issue_types: Vec<IssueType> = Vec::new();

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/projects/{}/issueTypes", project_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_types))
            .mount(&server)
            .await;

        let result = project_api
            .get_issue_type_list(ProjectIdOrKey::from_str(project_key).unwrap())
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_issue_type_list_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = 999; // Non-existent project

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such project.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/projects/{}/issueTypes", project_id)))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let result = project_api
            .get_issue_type_list(ProjectId::new(project_id))
            .await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such project.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }
}
