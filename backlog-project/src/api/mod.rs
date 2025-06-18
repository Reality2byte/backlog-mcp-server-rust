use backlog_api_core::Result;
use backlog_core::{ProjectIdOrKey, identifier::CategoryId};
use client::Client;

use crate::requests::{GetProjectListResponse, GetProjectParams, GetProjectResponse};
use backlog_domain_models::{Category, IssueType, Milestone, Priority, Resolution, Status};

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

    /// Gets the list of categories for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/categories`.
    pub async fn get_category_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<Vec<Category>> {
        let path = format!("/api/v2/projects/{}/categories", project_id_or_key.into());
        self.0.get(&path).await
    }

    /// Gets the list of priorities.
    ///
    /// Corresponds to `GET /api/v2/priorities`.
    pub async fn get_priority_list(&self) -> Result<Vec<Priority>> {
        self.0.get("/api/v2/priorities").await
    }

    /// Gets the list of resolutions.
    ///
    /// Corresponds to `GET /api/v2/resolutions`.
    pub async fn get_resolution_list(&self) -> Result<Vec<Resolution>> {
        self.0.get("/api/v2/resolutions").await
    }

    /// Gets the project icon image data.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/image`.
    pub async fn get_project_icon(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<Vec<u8>> {
        let path = format!("/api/v2/projects/{}/image", project_id_or_key.into());
        let downloaded_file = self.0.download_file_raw(&path).await?;
        Ok(downloaded_file.bytes.to_vec())
    }

    /// Adds a category to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/categories`.
    #[cfg(feature = "writable")]
    pub async fn add_category(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        params: &crate::requests::AddCategoryParams,
    ) -> Result<Category> {
        let path = format!("/api/v2/projects/{}/categories", project_id_or_key.into());
        let params_vec: Vec<(String, String)> = params.into();
        self.0.post(&path, &params_vec).await
    }

    /// Updates a category in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/categories/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_category(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        category_id: impl Into<CategoryId>,
        params: &crate::requests::UpdateCategoryParams,
    ) -> Result<Category> {
        let path = format!(
            "/api/v2/projects/{}/categories/{}",
            project_id_or_key.into(),
            category_id.into()
        );
        let params_vec: Vec<(String, String)> = params.into();
        self.0.patch(&path, &params_vec).await
    }

    /// Deletes a category from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/categories/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_category(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        category_id: impl Into<CategoryId>,
    ) -> Result<Category> {
        let path = format!(
            "/api/v2/projects/{}/categories/{}",
            project_id_or_key.into(),
            category_id.into()
        );
        self.0.delete(&path).await
    }

    /// Adds an issue type to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/issueTypes`.
    #[cfg(feature = "writable")]
    pub async fn add_issue_type(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        params: &crate::requests::AddIssueTypeParams,
    ) -> Result<IssueType> {
        let path = format!("/api/v2/projects/{}/issueTypes", project_id_or_key.into());
        let params_vec: Vec<(String, String)> = params.into();
        self.0.post(&path, &params_vec).await
    }

    /// Deletes an issue type from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/issueTypes/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_issue_type(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        issue_type_id: impl Into<backlog_core::identifier::IssueTypeId>,
        params: &crate::requests::DeleteIssueTypeParams,
    ) -> Result<IssueType> {
        let path = format!(
            "/api/v2/projects/{}/issueTypes/{}",
            project_id_or_key.into(),
            issue_type_id.into()
        );
        let params_vec: Vec<(String, String)> = params.into();
        self.0.delete_with_params(&path, &params_vec).await
    }
}

type GetVersionMilestoneListResponse = Vec<Milestone>;

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_api_core::Error as ApiError;
    use backlog_core::identifier::{
        CategoryId, IssueTypeId, MilestoneId, PriorityId, ProjectId, ResolutionId, StatusId,
    };
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

    #[tokio::test]
    async fn test_get_category_list_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_categories = vec![
            Category {
                id: CategoryId::new(1),
                project_id,
                name: "Development".to_string(),
                display_order: 0,
            },
            Category {
                id: CategoryId::new(2),
                project_id,
                name: "Bug".to_string(),
                display_order: 1,
            },
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/projects/123/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_categories))
            .mount(&server)
            .await;

        let result = project_api.get_category_list(project_id).await;
        assert!(result.is_ok());
        let categories = result.unwrap();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name, "Development");
        assert_eq!(categories[1].name, "Bug");
    }

    #[tokio::test]
    async fn test_get_category_list_empty() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let expected_categories: Vec<Category> = Vec::new();

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/projects/{}/categories", project_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_categories))
            .mount(&server)
            .await;

        let result = project_api
            .get_category_list(ProjectIdOrKey::from_str(project_key).unwrap())
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_category_list_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = 999;

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
            .and(path(format!("/api/v2/projects/{}/categories", project_id)))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let result = project_api
            .get_category_list(ProjectId::new(project_id))
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
    async fn test_get_priority_list_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);

        let expected_priorities = vec![
            Priority {
                id: PriorityId::new(2),
                name: "High".to_string(),
            },
            Priority {
                id: PriorityId::new(3),
                name: "Normal".to_string(),
            },
            Priority {
                id: PriorityId::new(4),
                name: "Low".to_string(),
            },
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/priorities"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_priorities))
            .mount(&server)
            .await;

        let result = project_api.get_priority_list().await;
        assert!(result.is_ok());
        let priorities = result.unwrap();
        assert_eq!(priorities.len(), 3);
        assert_eq!(priorities[0].name, "High");
        assert_eq!(priorities[1].name, "Normal");
        assert_eq!(priorities[2].name, "Low");
    }

    #[tokio::test]
    async fn test_get_resolution_list_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);

        let expected_resolutions = vec![
            Resolution {
                id: ResolutionId::new(0),
                name: "Fixed".to_string(),
            },
            Resolution {
                id: ResolutionId::new(1),
                name: "Won't Fix".to_string(),
            },
            Resolution {
                id: ResolutionId::new(2),
                name: "Invalid".to_string(),
            },
            Resolution {
                id: ResolutionId::new(3),
                name: "Duplication".to_string(),
            },
            Resolution {
                id: ResolutionId::new(4),
                name: "Cannot Reproduce".to_string(),
            },
        ];

        Mock::given(method("GET"))
            .and(path("/api/v2/resolutions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_resolutions))
            .mount(&server)
            .await;

        let result = project_api.get_resolution_list().await;
        assert!(result.is_ok());
        let resolutions = result.unwrap();
        assert_eq!(resolutions.len(), 5);
        assert_eq!(resolutions[0].name, "Fixed");
        assert_eq!(resolutions[1].name, "Won't Fix");
        assert_eq!(resolutions[2].name, "Invalid");
        assert_eq!(resolutions[3].name, "Duplication");
        assert_eq!(resolutions[4].name, "Cannot Reproduce");
    }

    #[tokio::test]
    async fn test_get_priority_list_empty() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);

        let expected_priorities: Vec<Priority> = Vec::new();

        Mock::given(method("GET"))
            .and(path("/api/v2/priorities"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_priorities))
            .mount(&server)
            .await;

        let result = project_api.get_priority_list().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_resolution_list_empty() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);

        let expected_resolutions: Vec<Resolution> = Vec::new();

        Mock::given(method("GET"))
            .and(path("/api/v2/resolutions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_resolutions))
            .mount(&server)
            .await;

        let result = project_api.get_resolution_list().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_category_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_category = Category {
            id: CategoryId::new(1),
            project_id,
            name: "Development".to_string(),
            display_order: 0,
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_category))
            .mount(&server)
            .await;

        let params = crate::requests::AddCategoryParams {
            name: "Development".to_string(),
        };
        let result = project_api.add_category(project_id, &params).await;
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Development");
        assert_eq!(category.id, CategoryId::new(1));
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_category_duplicate_name_error() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Category name already exists.",
                    "code": 12,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/projects/{}/categories", project_key)))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = crate::requests::AddCategoryParams {
            name: "Existing Category".to_string(),
        };
        let result = project_api
            .add_category(ProjectIdOrKey::from_str(project_key).unwrap(), &params)
            .await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 400);
            assert_eq!(errors[0].message, "Category name already exists.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_category_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let category_id = CategoryId::new(5);

        let expected_category = Category {
            id: category_id,
            project_id,
            name: "Development".to_string(),
            display_order: 0,
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/categories/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_category))
            .mount(&server)
            .await;

        let result = project_api.delete_category(project_id, category_id).await;
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Development");
        assert_eq!(category.id, category_id);
        assert_eq!(category.project_id, project_id);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_category_with_project_key() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";
        let category_id = CategoryId::new(10);

        let expected_category = Category {
            id: category_id,
            project_id: ProjectId::new(456),
            name: "Bug Tracking".to_string(),
            display_order: 1,
        };

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/projects/{}/categories/10",
                project_key
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_category))
            .mount(&server)
            .await;

        let result = project_api
            .delete_category(ProjectIdOrKey::from_str(project_key).unwrap(), category_id)
            .await;
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Bug Tracking");
        assert_eq!(category.id, category_id);
        assert_eq!(category.display_order, 1);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_category_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let category_id = CategoryId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such category.",
                    "code": 7,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/categories/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let result = project_api.delete_category(project_id, category_id).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such category.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_category_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let category_id = CategoryId::new(1);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such project.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/999/categories/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let result = project_api.delete_category(project_id, category_id).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such project.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_category_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let category_id = CategoryId::new(456);

        let expected_category = Category {
            id: category_id,
            project_id,
            name: "Updated Category".to_string(),
            display_order: 1,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/categories/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_category))
            .mount(&server)
            .await;

        let params = crate::requests::UpdateCategoryParams {
            name: "Updated Category".to_string(),
        };

        let result = project_api
            .update_category(project_id, category_id, &params)
            .await;
        assert!(result.is_ok(), "Result was: {:?}", result);
        let category = result.unwrap();
        assert_eq!(category.id, category_id);
        assert_eq!(category.project_id, project_id);
        assert_eq!(category.name, "Updated Category");
        assert_eq!(category.display_order, 1);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_category_with_project_key_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";
        let category_id = CategoryId::new(789);

        let expected_category = Category {
            id: category_id,
            project_id: ProjectId::new(123),
            name: "Category with Key".to_string(),
            display_order: 2,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST_PROJECT/categories/789"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_category))
            .mount(&server)
            .await;

        let params = crate::requests::UpdateCategoryParams {
            name: "Category with Key".to_string(),
        };

        let result = project_api
            .update_category(
                ProjectIdOrKey::from_str(project_key).unwrap(),
                category_id,
                &params,
            )
            .await;
        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Category with Key");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_category_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let category_id = CategoryId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such category.",
                    "code": 8,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/categories/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = crate::requests::UpdateCategoryParams {
            name: "Non-existent Category".to_string(),
        };

        let result = project_api
            .update_category(project_id, category_id, &params)
            .await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such category.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_category_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let category_id = CategoryId::new(1);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such project.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/999/categories/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = crate::requests::UpdateCategoryParams {
            name: "Test Category".to_string(),
        };

        let result = project_api
            .update_category(project_id, category_id, &params)
            .await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such project.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_issue_type_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_issue_type = IssueType {
            id: backlog_core::identifier::IssueTypeId::new(1),
            project_id,
            name: "Bug".to_string(),
            color: "#990000".to_string(),
            display_order: 0,
            template_summary: Some("Subject".to_string()),
            template_description: Some("Description".to_string()),
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/issueTypes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&server)
            .await;

        let params = crate::requests::AddIssueTypeParams {
            name: "Bug".to_string(),
            color: backlog_domain_models::IssueTypeColor::DarkRed,
            template_summary: Some("Subject".to_string()),
            template_description: Some("Description".to_string()),
        };
        let result = project_api.add_issue_type(project_id, &params).await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Bug");
        assert_eq!(issue_type.color, "#990000");
        assert_eq!(issue_type.id, IssueTypeId::new(1));
        assert_eq!(issue_type.template_summary, Some("Subject".to_string()));
        assert_eq!(
            issue_type.template_description,
            Some("Description".to_string())
        );
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_issue_type_minimal_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let expected_issue_type = IssueType {
            id: backlog_core::identifier::IssueTypeId::new(2),
            project_id: ProjectId::new(456),
            name: "Task".to_string(),
            color: "#009900".to_string(),
            display_order: 1,
            template_summary: None,
            template_description: None,
        };

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/projects/{}/issueTypes", project_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&server)
            .await;

        let params = crate::requests::AddIssueTypeParams {
            name: "Task".to_string(),
            color: backlog_domain_models::IssueTypeColor::Green,
            template_summary: None,
            template_description: None,
        };
        let result = project_api
            .add_issue_type(ProjectIdOrKey::from_str(project_key).unwrap(), &params)
            .await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Task");
        assert_eq!(issue_type.color, "#009900");
        assert_eq!(issue_type.template_summary, None);
        assert_eq!(issue_type.template_description, None);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_issue_type_duplicate_name_error() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Issue type name already exists.",
                    "code": 14,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/projects/{}/issueTypes", project_key)))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = crate::requests::AddIssueTypeParams {
            name: "Existing Issue Type".to_string(),
            color: backlog_domain_models::IssueTypeColor::Red,
            template_summary: None,
            template_description: None,
        };
        let result = project_api
            .add_issue_type(ProjectIdOrKey::from_str(project_key).unwrap(), &params)
            .await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 400);
            assert_eq!(errors[0].message, "Issue type name already exists.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_issue_type_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such project.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/999/issueTypes"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = crate::requests::AddIssueTypeParams {
            name: "New Issue Type".to_string(),
            color: backlog_domain_models::IssueTypeColor::Blue,
            template_summary: None,
            template_description: None,
        };
        let result = project_api.add_issue_type(project_id, &params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such project.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_issue_type_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(456);
        let substitute_issue_type_id = backlog_core::identifier::IssueTypeId::new(789);

        let expected_issue_type = IssueType {
            id: issue_type_id,
            project_id,
            name: "Deleted Issue Type".to_string(),
            color: "#ff0000".to_string(),
            display_order: 2,
            template_summary: None,
            template_description: None,
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/issueTypes/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&server)
            .await;

        let params = crate::requests::DeleteIssueTypeParams {
            substitute_issue_type_id,
        };
        let result = project_api
            .delete_issue_type(project_id, issue_type_id, &params)
            .await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Deleted Issue Type");
        assert_eq!(issue_type.id, issue_type_id);
        assert_eq!(issue_type.project_id, project_id);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_issue_type_with_project_key() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(100);
        let substitute_issue_type_id = backlog_core::identifier::IssueTypeId::new(200);

        let expected_issue_type = IssueType {
            id: issue_type_id,
            project_id: ProjectId::new(456),
            name: "Task Type".to_string(),
            color: "#00ff00".to_string(),
            display_order: 1,
            template_summary: Some("Task Template".to_string()),
            template_description: Some("Task Description".to_string()),
        };

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/api/v2/projects/{}/issueTypes/100",
                project_key
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&server)
            .await;

        let params = crate::requests::DeleteIssueTypeParams {
            substitute_issue_type_id,
        };
        let result = project_api
            .delete_issue_type(
                ProjectIdOrKey::from_str(project_key).unwrap(),
                issue_type_id,
                &params,
            )
            .await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Task Type");
        assert_eq!(issue_type.id, issue_type_id);
        assert_eq!(issue_type.display_order, 1);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_issue_type_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(999);
        let substitute_issue_type_id = backlog_core::identifier::IssueTypeId::new(1);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such issue type.",
                    "code": 13,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/issueTypes/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = crate::requests::DeleteIssueTypeParams {
            substitute_issue_type_id,
        };
        let result = project_api
            .delete_issue_type(project_id, issue_type_id, &params)
            .await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such issue type.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_issue_type_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(1);
        let substitute_issue_type_id = backlog_core::identifier::IssueTypeId::new(2);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such project.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/999/issueTypes/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = crate::requests::DeleteIssueTypeParams {
            substitute_issue_type_id,
        };
        let result = project_api
            .delete_issue_type(project_id, issue_type_id, &params)
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
