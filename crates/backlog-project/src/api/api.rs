use backlog_api_core::Result;
use backlog_core::ProjectIdOrKey;
use client::Client;
use backlog_domain_models::{Category, IssueType, Milestone, Status};

use crate::api::{
    get_project_list::{GetProjectListParams, GetProjectListResponse}, GetCategoryListParams, GetCategoryListResponse, GetIssueTypeListParams, GetIssueTypeListResponse, GetPriorityListParams, GetPriorityListResponse, GetProjectDetailParams, GetProjectDetailResponse, GetProjectIconParams, GetResolutionListParams, GetResolutionListResponse, GetStatusListParams, GetStatusListResponse, GetVersionMilestoneListParams, GetVersionMilestoneListResponse};
#[cfg(feature = "writable")]
use crate::{api::{AddCategoryResponse, DeleteStatusParams, UpdateStatusOrderParams, UpdateStatusParams}, AddCategoryParams, AddIssueTypeParams, AddStatusParams, AddVersionParams, DeleteCategoryParams, DeleteIssueTypeParams, DeleteVersionParams, UpdateCategoryParams, UpdateIssueTypeParams, UpdateVersionParams};


pub struct ProjectApi(Client);

impl ProjectApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Gets the list of projects.
    /// Corresponds to `GET /api/v2/projects`.
    pub async fn get_project_list(
        &self,
        params: GetProjectListParams,
    ) -> Result<GetProjectListResponse> {
        self.0.execute(params).await
    }

    /// Gets a project by its ID or key.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey`.
    pub async fn get_project(&self, params: GetProjectDetailParams) -> Result<GetProjectDetailResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of statuses for a project.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/statuses`.
    pub async fn get_status_list(&self, params: GetStatusListParams) -> Result<GetStatusListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of issue types for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/issueTypes`.
    pub async fn get_issue_type_list(
        &self,
        params: GetIssueTypeListParams,
    ) -> Result<GetIssueTypeListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of version milestones for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/versions`.
    pub async fn get_version_milestone_list(
        &self,
        params: GetVersionMilestoneListParams,
    ) -> Result<GetVersionMilestoneListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of categories for a project.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/categories`.
    pub async fn get_category_list(&self, params: GetCategoryListParams) -> Result<GetCategoryListResponse> {
        self.0.execute(params).await
    }

    /// Gets the list of priorities.
    ///
    /// Corresponds to `GET /api/v2/priorities`.
    pub async fn get_priority_list(&self) -> Result<GetPriorityListResponse> {
        self.0.execute(GetPriorityListParams).await
    }

    /// Gets the list of resolutions.
    ///
    /// Corresponds to `GET /api/v2/resolutions`.
    pub async fn get_resolution_list(&self) -> Result<GetResolutionListResponse> {
        self.0.execute(GetResolutionListParams).await
    }

    /// Gets the project icon image data.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/image`.
    pub async fn get_project_icon(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<Vec<u8>> {
        let params = GetProjectIconParams::new(project_id_or_key);
        let downloaded_file = self.0.download_file(params).await?;
        Ok(downloaded_file.bytes.to_vec())
    }

    /// Adds a category to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/categories`.
    #[cfg(feature = "writable")]
    pub async fn add_category(
        &self,
        params: AddCategoryParams,
    ) -> Result<AddCategoryResponse> {
        self.0.execute(params).await
    }

    /// Updates a category in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/categories/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_category(
        &self,
        params: UpdateCategoryParams,
    ) -> Result<Category> {
        self.0.execute(params).await
    }

    /// Deletes a category from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/categories/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_category(
        &self,
        params: DeleteCategoryParams,
    ) -> Result<Category> {
        self.0.execute(params).await
    }

    /// Adds an issue type to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/issueTypes`.
    #[cfg(feature = "writable")]
    pub async fn add_issue_type(
        &self,
        params: AddIssueTypeParams,
    ) -> Result<IssueType> {
        self.0.execute(params).await
    }

    /// Deletes an issue type from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/issueTypes/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_issue_type(
        &self,
        params: DeleteIssueTypeParams,
    ) -> Result<IssueType> {
        self.0.execute(params).await
    }

    /// Updates an issue type in a project.
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/issueTypes/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_issue_type(
        &self,
        params: UpdateIssueTypeParams,
    ) -> Result<IssueType> {
        self.0.execute(params).await
    }

    /// Adds a version/milestone to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/versions`.
    #[cfg(feature = "writable")]
    pub async fn add_version(
        &self,
        params: AddVersionParams,
    ) -> Result<Milestone> {
        self.0.execute(params).await
    }

    /// Updates a version/milestone in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/versions/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_version(
        &self,
        params: UpdateVersionParams,
    ) -> Result<Milestone> {
        self.0.execute(params).await
    }

    /// Deletes a version/milestone from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/versions/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_version(
        &self,
        params: DeleteVersionParams,
    ) -> Result<Milestone> {
        self.0.execute(params).await
    }

    /// Adds a status to a project.
    ///
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/statuses`.
    #[cfg(feature = "writable")]
    pub async fn add_status(&self, params: AddStatusParams) -> Result<Status> {
        self.0.execute(params).await
    }

    /// Updates a status in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/statuses/:id`.
    #[cfg(feature = "writable")]
    pub async fn update_status(
        &self,
        params: UpdateStatusParams,
    ) -> Result<Status> {
        self.0.execute(params).await
    }

    /// Deletes a status from a project.
    ///
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/statuses/:id`.
    #[cfg(feature = "writable")]
    pub async fn delete_status(
        &self,
        params: DeleteStatusParams,
    ) -> Result<Status> {
        self.0.execute(params).await
    }

    /// Updates the display order of statuses in a project.
    ///
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/statuses/updateDisplayOrder`.
    #[cfg(feature = "writable")]
    pub async fn update_status_order(
        &self,
        params: UpdateStatusOrderParams,
    ) -> Result<Vec<Status>> {
        self.0.execute(params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_api_core::Error as ApiError;
    use backlog_core::identifier::{
        CategoryId, IssueTypeId, MilestoneId, PriorityId, ProjectId, ResolutionId, StatusId,
    };
    use backlog_domain_models::Milestone;
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
        let params = GetVersionMilestoneListParams::new(project_id_or_key.clone());
        let result = issue_api.get_version_milestone_list(params).await;
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
        let params = GetVersionMilestoneListParams::new(project_id_or_key.clone());
        let result = issue_api.get_version_milestone_list(params).await;
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

        let params = GetStatusListParams::new(project_id);
        let result = project_api.get_status_list(params).await;
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
            .get_status_list(GetStatusListParams::new(
                ProjectIdOrKey::from_str(project_key).unwrap(),
            ))
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
            .get_status_list(GetStatusListParams::new(ProjectId::new(project_id)))
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

        let params = GetIssueTypeListParams::new(project_id);
        let result = project_api.get_issue_type_list(params).await;
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
            .get_issue_type_list(GetIssueTypeListParams::new(
                ProjectIdOrKey::from_str(project_key).unwrap(),
            ))
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
            .get_issue_type_list(GetIssueTypeListParams::new(ProjectId::new(project_id)))
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

        let params = GetCategoryListParams::new(project_id);
        let result = project_api.get_category_list(params).await;
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
            .get_category_list(GetCategoryListParams::new(
                ProjectIdOrKey::from_str(project_key).unwrap(),
            ))
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
            .get_category_list(GetCategoryListParams::new(ProjectId::new(project_id)))
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

        let params = AddCategoryParams::new(project_id, "Development");
        let result = project_api.add_category(params).await;
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

        let params = AddCategoryParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            "Existing Category",
        );
        let result = project_api.add_category(params).await;
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

        let result = project_api
            .delete_category(DeleteCategoryParams::new(
                project_id,
                category_id,
            ))
            .await;
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
            .delete_category(DeleteCategoryParams::new(
                ProjectIdOrKey::from_str(project_key).unwrap(),
                category_id,
            ))
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

        let result = project_api
            .delete_category(DeleteCategoryParams::new(
                project_id,
                category_id,
            ))
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

        let result = project_api
            .delete_category(DeleteCategoryParams::new(
                project_id,
                category_id,
            ))
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

        let params =
            UpdateCategoryParams::new(project_id, category_id, "Updated Category");

        let result = project_api.update_category(params).await;
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

        let params = UpdateCategoryParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            category_id,
            "Category with Key",
        );

        let result = project_api.update_category(params).await;
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

        let params = UpdateCategoryParams::new(
            project_id,
            category_id,
            "Non-existent Category",
        );

        let result = project_api.update_category(params).await;
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

        let params =
            UpdateCategoryParams::new(project_id, category_id, "Test Category");

        let result = project_api.update_category(params).await;
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

        let mut params = AddIssueTypeParams::new(
            project_id,
            "Bug",
            backlog_domain_models::IssueTypeColor::DarkRed,
        );
        params.template_summary = Some("Subject".to_string());
        params.template_description = Some("Description".to_string());
        let result = project_api.add_issue_type(params).await;
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

        let params = AddIssueTypeParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            "Task",
            backlog_domain_models::IssueTypeColor::Green,
        );
        let result = project_api.add_issue_type(params).await;
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

        let params = AddIssueTypeParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            "Existing Issue Type",
            backlog_domain_models::IssueTypeColor::Red,
        );
        let result = project_api.add_issue_type(params).await;
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

        let params = AddIssueTypeParams::new(
            project_id,
            "New Issue Type",
            backlog_domain_models::IssueTypeColor::Blue,
        );
        let result = project_api.add_issue_type(params).await;
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

        let params = DeleteIssueTypeParams::new(
            project_id,
            issue_type_id,
            substitute_issue_type_id,
        );
        let result = project_api.delete_issue_type(params).await;
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

        let params = DeleteIssueTypeParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            issue_type_id,
            substitute_issue_type_id,
        );
        let result = project_api.delete_issue_type(params).await;
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

        let params = DeleteIssueTypeParams::new(
            project_id,
            issue_type_id,
            substitute_issue_type_id,
        );
        let result = project_api.delete_issue_type(params).await;
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

        let params = DeleteIssueTypeParams::new(
            project_id,
            issue_type_id,
            substitute_issue_type_id,
        );
        let result = project_api.delete_issue_type(params).await;
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
    async fn test_update_issue_type_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(456);

        let expected_issue_type = IssueType {
            id: issue_type_id,
            project_id,
            name: "Updated Bug Type".to_string(),
            color: "#009900".to_string(),
            display_order: 1,
            template_summary: Some("Updated Summary".to_string()),
            template_description: Some("Updated Description".to_string()),
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/issueTypes/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&server)
            .await;

        let mut params = UpdateIssueTypeParams::new(project_id, issue_type_id);
        params.name = Some("Updated Bug Type".to_string());
        params.color = Some(backlog_domain_models::IssueTypeColor::Green);
        params.template_summary = Some("Updated Summary".to_string());
        params.template_description = Some("Updated Description".to_string());

        let result = project_api.update_issue_type(params).await;
        assert!(result.is_ok(), "Result was: {:?}", result);
        let issue_type = result.unwrap();
        assert_eq!(issue_type.id, issue_type_id);
        assert_eq!(issue_type.project_id, project_id);
        assert_eq!(issue_type.name, "Updated Bug Type");
        assert_eq!(issue_type.color, "#009900");
        assert_eq!(
            issue_type.template_summary,
            Some("Updated Summary".to_string())
        );
        assert_eq!(
            issue_type.template_description,
            Some("Updated Description".to_string())
        );
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_issue_type_partial_update() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(456);

        let expected_issue_type = IssueType {
            id: issue_type_id,
            project_id,
            name: "Only Name Updated".to_string(),
            color: "#990000".to_string(),
            display_order: 1,
            template_summary: None,
            template_description: None,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/issueTypes/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&server)
            .await;

        let mut params = UpdateIssueTypeParams::new(project_id, issue_type_id);
        params.name = Some("Only Name Updated".to_string());

        let result = project_api.update_issue_type(params).await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Only Name Updated");
        assert_eq!(issue_type.id, issue_type_id);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_issue_type_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such issue type.",
                    "code": 13,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/issueTypes/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let mut params = UpdateIssueTypeParams::new(project_id, issue_type_id);
        params.name = Some("Non-existent Issue Type".to_string());

        let result = project_api.update_issue_type(params).await;
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
    async fn test_update_issue_type_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(456);

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
            .and(path("/api/v2/projects/999/issueTypes/456"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let mut params = UpdateIssueTypeParams::new(project_id, issue_type_id);
        params.name = Some("Issue Type".to_string());

        let result = project_api.update_issue_type(params).await;
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
    async fn test_update_issue_type_with_project_key() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TESTPROJ";
        let issue_type_id = backlog_core::identifier::IssueTypeId::new(456);

        let expected_issue_type = IssueType {
            id: issue_type_id,
            project_id: ProjectId::new(123),
            name: "Updated with Key".to_string(),
            color: "#ff3265".to_string(),
            display_order: 2,
            template_summary: Some("Key-based update".to_string()),
            template_description: None,
        };

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/issueTypes/456",
                project_key
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_issue_type))
            .mount(&server)
            .await;

        let mut params = UpdateIssueTypeParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            issue_type_id,
        );
        params.name = Some("Updated with Key".to_string());
        params.color = Some(backlog_domain_models::IssueTypeColor::Pink);
        params.template_summary = Some("Key-based update".to_string());

        let result = project_api.update_issue_type(params).await;
        assert!(result.is_ok());
        let issue_type = result.unwrap();
        assert_eq!(issue_type.name, "Updated with Key");
        assert_eq!(issue_type.color, "#ff3265");
        assert_eq!(issue_type.id, issue_type_id);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_version_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_milestone = Milestone {
            id: MilestoneId::new(1),
            project_id,
            name: "Version 1.0".to_string(),
            description: Some("Initial release".to_string()),
            start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
            archived: false,
            display_order: Some(1),
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/versions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_milestone))
            .mount(&server)
            .await;

        let mut params = AddVersionParams::new(project_id, "Version 1.0");
        params.description = Some("Initial release".to_string());
        params.start_date = Some("2023-01-01".to_string());
        params.release_due_date = Some("2023-01-31".to_string());
        let result = project_api.add_version(params).await;
        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.name, "Version 1.0");
        assert_eq!(milestone.id, MilestoneId::new(1));
        assert_eq!(milestone.description, Some("Initial release".to_string()));
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_version_minimal_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let expected_milestone = Milestone {
            id: MilestoneId::new(2),
            project_id: ProjectId::new(456),
            name: "Simple Version".to_string(),
            description: None,
            start_date: None,
            release_due_date: None,
            archived: false,
            display_order: Some(2),
        };

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/projects/{}/versions", project_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_milestone))
            .mount(&server)
            .await;

        let params = AddVersionParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            "Simple Version",
        );
        let result = project_api.add_version(params).await;
        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.name, "Simple Version");
        assert_eq!(milestone.description, None);
        assert_eq!(milestone.start_date, None);
        assert_eq!(milestone.release_due_date, None);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_version_duplicate_name_error() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Version name already exists.",
                    "code": 15,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/projects/{}/versions", project_key)))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = AddVersionParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            "Existing Version",
        );
        let result = project_api.add_version(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 400);
            assert_eq!(errors[0].message, "Version name already exists.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_version_project_not_found() {
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
            .and(path("/api/v2/projects/999/versions"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let mut params = AddVersionParams::new(project_id, "New Version");
        params.description = Some("Description".to_string());
        let result = project_api.add_version(params).await;
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
    async fn test_update_version_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let version_id = MilestoneId::new(456);

        let expected_milestone = Milestone {
            id: version_id,
            project_id,
            name: "Updated Version".to_string(),
            description: Some("Updated description".to_string()),
            start_date: Some(chrono::Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()),
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2025, 1, 31, 0, 0, 0).unwrap()),
            archived: true,
            display_order: Some(1),
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/versions/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_milestone))
            .mount(&server)
            .await;

        let mut params =
            UpdateVersionParams::new(project_id, version_id, "Updated Version");
        params.description = Some("Updated description".to_string());
        params.start_date = Some("2025-01-01".to_string());
        params.release_due_date = Some("2025-01-31".to_string());
        params.archived = Some(true);

        let result = project_api.update_version(params).await;
        assert!(result.is_ok(), "Result was: {:?}", result);
        let milestone = result.unwrap();
        assert_eq!(milestone.id, version_id);
        assert_eq!(milestone.project_id, project_id);
        assert_eq!(milestone.name, "Updated Version");
        assert_eq!(
            milestone.description,
            Some("Updated description".to_string())
        );
        assert!(milestone.archived);
        assert_eq!(milestone.display_order, Some(1));
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_version_minimal_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";
        let version_id = MilestoneId::new(789);

        let expected_milestone = Milestone {
            id: version_id,
            project_id: ProjectId::new(456),
            name: "Name Only Update".to_string(),
            description: None,
            start_date: None,
            release_due_date: None,
            archived: false,
            display_order: Some(2),
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST_PROJECT/versions/789"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_milestone))
            .mount(&server)
            .await;

        let params = UpdateVersionParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            version_id,
            "Name Only Update",
        );

        let result = project_api.update_version(params).await;
        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.name, "Name Only Update");
        assert_eq!(milestone.description, None);
        assert_eq!(milestone.start_date, None);
        assert_eq!(milestone.release_due_date, None);
        assert!(!milestone.archived);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_version_archive_only() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let version_id = MilestoneId::new(456);

        let expected_milestone = Milestone {
            id: version_id,
            project_id,
            name: "Archived Version".to_string(),
            description: Some("This version is now archived".to_string()),
            start_date: None,
            release_due_date: None,
            archived: true,
            display_order: Some(3),
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/versions/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_milestone))
            .mount(&server)
            .await;

        let mut params =
            UpdateVersionParams::new(project_id, version_id, "Archived Version");
        params.description = Some("This version is now archived".to_string());
        params.archived = Some(true);

        let result = project_api.update_version(params).await;
        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.name, "Archived Version");
        assert!(milestone.archived);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_version_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let version_id = MilestoneId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such version.",
                    "code": 16,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/versions/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UpdateVersionParams::new(
            project_id,
            version_id,
            "Non-existent Version",
        );

        let result = project_api.update_version(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such version.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_version_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let version_id = MilestoneId::new(1);

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
            .and(path("/api/v2/projects/999/versions/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params =
            UpdateVersionParams::new(project_id, version_id, "Test Version");

        let result = project_api.update_version(params).await;
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
    async fn test_delete_version_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let version_id = MilestoneId::new(1);

        let expected_milestone = Milestone {
            id: version_id,
            project_id,
            name: "Version 1.0".to_string(),
            description: Some("Initial release".to_string()),
            start_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 31, 0, 0, 0).unwrap()),
            archived: false,
            display_order: Some(1),
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/versions/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_milestone))
            .mount(&server)
            .await;

        let params = DeleteVersionParams::new(project_id, version_id);
        let result = project_api.delete_version(params).await;
        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.name, "Version 1.0");
        assert_eq!(milestone.id, version_id);
        assert_eq!(milestone.project_id, project_id);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_version_with_project_key() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";
        let version_id = MilestoneId::new(5);

        let expected_milestone = Milestone {
            id: version_id,
            project_id: ProjectId::new(456),
            name: "Version 2.0".to_string(),
            description: Some("Major update".to_string()),
            start_date: None,
            release_due_date: Some(chrono::Utc.with_ymd_and_hms(2024, 12, 31, 0, 0, 0).unwrap()),
            archived: true,
            display_order: Some(2),
        };

        Mock::given(method("DELETE"))
            .and(path(format!("/api/v2/projects/{}/versions/5", project_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_milestone))
            .mount(&server)
            .await;

        let params = DeleteVersionParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            version_id,
        );
        let result = project_api.delete_version(params).await;
        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.name, "Version 2.0");
        assert_eq!(milestone.id, version_id);
        assert!(milestone.archived);
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_version_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let version_id = MilestoneId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such version.",
                    "code": 8,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/versions/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = DeleteVersionParams::new(project_id, version_id);
        let result = project_api.delete_version(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such version.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_version_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let version_id = MilestoneId::new(1);

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
            .and(path("/api/v2/projects/999/versions/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = DeleteVersionParams::new(project_id, version_id);
        let result = project_api.delete_version(params).await;
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
    async fn test_add_status_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_status = Status {
            id: StatusId::new(101),
            project_id,
            name: "レビュー待ち".to_string(),
            color: "#e87758".to_string(),
            display_order: 3999,
        };

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_status))
            .mount(&server)
            .await;

        let params = AddStatusParams::new(
            project_id,
            "レビュー待ち",
            backlog_domain_models::StatusColor::Coral,
        );
        let result = project_api.add_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "レビュー待ち");
        assert_eq!(status.id, StatusId::new(101));
        assert_eq!(status.color, "#e87758");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_status_duplicate_name_error() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Status name already exists.",
                    "code": 13,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path(format!("/api/v2/projects/{}/statuses", project_key)))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = AddStatusParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            "処理中",
            backlog_domain_models::StatusColor::Blue,
        );
        let result = project_api.add_status(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 400);
            assert_eq!(errors[0].message, "Status name already exists.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_add_status_project_not_found() {
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
            .and(path("/api/v2/projects/999/statuses"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = AddStatusParams::new(
            project_id,
            "テスト状態",
            backlog_domain_models::StatusColor::Green,
        );

        let result = project_api.add_status(params).await;
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
    async fn test_add_status_limit_exceeded() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Status limit exceeded. Maximum 8 custom statuses allowed.",
                    "code": 14,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/projects/123/statuses"))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = AddStatusParams::new(
            project_id,
            "追加状態",
            backlog_domain_models::StatusColor::Orange,
        );

        let result = project_api.add_status(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 400);
            assert_eq!(
                errors[0].message,
                "Status limit exceeded. Maximum 8 custom statuses allowed."
            );
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let status_id = StatusId::new(101);

        let expected_status = Status {
            id: status_id,
            project_id,
            name: "更新されたレビュー".to_string(),
            color: "#3b9dbd".to_string(),
            display_order: 3999,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/statuses/101"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_status))
            .mount(&server)
            .await;

        let params = UpdateStatusParams::new(project_id, status_id)
            .name("更新されたレビュー")
            .color(backlog_domain_models::StatusColor::Blue);
        let result = project_api.update_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "更新されたレビュー");
        assert_eq!(status.id, status_id);
        assert_eq!(status.color, "#3b9dbd");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_partial_update_name_only() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_key = "TEST_PROJECT";
        let status_id = StatusId::new(101);

        let expected_status = Status {
            id: status_id,
            project_id: ProjectId::new(456),
            name: "名前のみ更新".to_string(),
            color: "#e87758".to_string(), // 色は変更されない
            display_order: 3999,
        };

        Mock::given(method("PATCH"))
            .and(path(format!(
                "/api/v2/projects/{}/statuses/101",
                project_key
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_status))
            .mount(&server)
            .await;

        let params = UpdateStatusParams::new(
            ProjectIdOrKey::from_str(project_key).unwrap(),
            status_id,
        )
        .name("名前のみ更新");
        let result = project_api.update_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "名前のみ更新");
        assert_eq!(status.color, "#e87758");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_partial_update_color_only() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let status_id = StatusId::new(101);

        let expected_status = Status {
            id: status_id,
            project_id,
            name: "レビュー待ち".to_string(), // 名前は変更されない
            color: "#4caf93".to_string(),
            display_order: 3999,
        };

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/statuses/101"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_status))
            .mount(&server)
            .await;

        let params = UpdateStatusParams::new(project_id, status_id)
            .color(backlog_domain_models::StatusColor::Green);
        let result = project_api.update_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "レビュー待ち");
        assert_eq!(status.color, "#4caf93");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let status_id = StatusId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such status.",
                    "code": 15,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/statuses/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UpdateStatusParams::new(project_id, status_id)
            .name("存在しないステータス")
            .color(backlog_domain_models::StatusColor::Red);

        let result = project_api.update_status(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such status.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let status_id = StatusId::new(101);

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
            .and(path("/api/v2/projects/999/statuses/101"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UpdateStatusParams::new(project_id, status_id)
            .name("プロジェクトなし");

        let result = project_api.update_status(params).await;
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
    async fn test_delete_status_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let status_id = StatusId::new(101);
        let substitute_status_id = StatusId::new(1);

        let expected_status = Status {
            id: status_id,
            project_id,
            name: "削除されたステータス".to_string(),
            color: "#e87758".to_string(),
            display_order: 3999,
        };

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/statuses/101"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_status))
            .mount(&server)
            .await;

        let params =
            DeleteStatusParams::new(project_id, status_id, substitute_status_id);
        let result = project_api.delete_status(params).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.name, "削除されたステータス");
        assert_eq!(status.id, status_id);
        assert_eq!(status.color, "#e87758");
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_status_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let status_id = StatusId::new(999);
        let substitute_status_id = StatusId::new(1);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such status.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/statuses/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params =
            DeleteStatusParams::new(project_id, status_id, substitute_status_id);
        let result = project_api.delete_status(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such status.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_delete_status_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(999);
        let status_id = StatusId::new(1);
        let substitute_status_id = StatusId::new(2);

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
            .and(path("/api/v2/projects/999/statuses/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params =
            DeleteStatusParams::new(project_id, status_id, substitute_status_id);
        let result = project_api.delete_status(params).await;
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
    async fn test_delete_status_substitute_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);
        let status_id = StatusId::new(101);
        let substitute_status_id = StatusId::new(999);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No such substitute status.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("DELETE"))
            .and(path("/api/v2/projects/123/statuses/101"))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params =
            DeleteStatusParams::new(project_id, status_id, substitute_status_id);
        let result = project_api.delete_status(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 400);
            assert_eq!(errors[0].message, "No such substitute status.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_order_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let expected_statuses = vec![
            Status {
                id: StatusId::new(1),
                project_id,
                name: "未対応".to_string(),
                color: "#ed8077".to_string(),
                display_order: 1,
            },
            Status {
                id: StatusId::new(3),
                project_id,
                name: "処理済み".to_string(),
                color: "#5eb5a6".to_string(),
                display_order: 2,
            },
            Status {
                id: StatusId::new(2),
                project_id,
                name: "処理中".to_string(),
                color: "#4488c5".to_string(),
                display_order: 3,
            },
            Status {
                id: StatusId::new(4),
                project_id,
                name: "完了".to_string(),
                color: "#b0be3c".to_string(),
                display_order: 4,
            },
        ];

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/statuses/updateDisplayOrder"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_statuses))
            .mount(&server)
            .await;

        let params = UpdateStatusOrderParams::new(
            project_id,
            vec![
                StatusId::new(1),
                StatusId::new(3),
                StatusId::new(2),
                StatusId::new(4),
            ],
        );
        let result = project_api.update_status_order(params).await;
        assert!(result.is_ok());
        let statuses = result.unwrap();
        assert_eq!(statuses.len(), 4);
        assert_eq!(statuses[0].id, StatusId::new(1));
        assert_eq!(statuses[1].id, StatusId::new(3));
        assert_eq!(statuses[2].id, StatusId::new(2));
        assert_eq!(statuses[3].id, StatusId::new(4));
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_order_incomplete_list() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let project_api = ProjectApi::new(client);
        let project_id = ProjectId::new(123);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Incomplete status list. All statuses must be included.",
                    "code": 6,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/statuses/updateDisplayOrder"))
            .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UpdateStatusOrderParams::new(
            project_id,
            vec![
                StatusId::new(1),
                StatusId::new(2),
                // Missing status IDs 3 and 4
            ],
        );
        let result = project_api.update_status_order(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 400);
            assert_eq!(
                errors[0].message,
                "Incomplete status list. All statuses must be included."
            );
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }

    #[cfg(feature = "writable")]
    #[tokio::test]
    async fn test_update_status_order_project_not_found() {
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

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/999/statuses/updateDisplayOrder"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UpdateStatusOrderParams::new(
            project_id,
            vec![
                StatusId::new(1),
                StatusId::new(2),
                StatusId::new(3),
                StatusId::new(4),
            ],
        );
        let result = project_api.update_status_order(params).await;
        assert!(result.is_err());
        if let Err(ApiError::HttpStatus { status, errors, .. }) = result {
            assert_eq!(status, 404);
            assert_eq!(errors[0].message, "No such project.");
        } else {
            panic!("Expected ApiError::HttpStatus, got {:?}", result);
        }
    }
}
