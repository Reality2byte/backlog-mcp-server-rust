use backlog_api_core::Result;
use backlog_core::ProjectIdOrKey;
use client::Client;

use crate::requests::{GetSharedFilesListParams, GetSharedFilesListResponse};

pub struct FileApi(Client);

impl FileApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Gets the list of shared files for a project directory.
    ///
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/files/metadata/:path`.
    pub async fn get_shared_files_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        path: impl AsRef<str>,
        params: GetSharedFilesListParams,
    ) -> Result<GetSharedFilesListResponse> {
        let project_id_or_key_val = project_id_or_key.into();
        let path_str = path.as_ref();
        let url = format!(
            "/api/v2/projects/{}/files/metadata/{}",
            project_id_or_key_val, path_str
        );
        self.0.get_with_params(&url, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_api_core::Error as ApiError;
    use backlog_core::identifier::{Identifier, ProjectId, SharedFileId, UserId};
    use backlog_core::{Language, Role};
    use chrono::TimeZone;
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_shared_files_list_success() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let file_api = FileApi::new(client);
        let project_id = ProjectId::new(123);
        let dir_path = "documents";

        let user = backlog_core::User {
            id: UserId::new(1),
            user_id: Some("testuser".to_string()),
            name: "Test User".to_string(),
            role_type: Role::Admin,
            lang: Some(Language::Japanese),
            mail_address: "test@example.com".to_string(),
            last_login_time: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
        };

        let expected_files = vec![crate::models::SharedFile {
            id: SharedFileId::new(1),
            project_id: ProjectId(123),
            dir: "/documents".to_string(),
            name: "test.txt".to_string(),
            created_user: user.clone(),
            created: chrono::Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(),
            updated_user: Some(user.clone()),
            updated: Some(chrono::Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap()),
            content: crate::models::FileContent::File { size: 1024 },
        }];

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/files/metadata/{}",
                project_id, dir_path
            )))
            .and(query_param("order", "desc"))
            .and(query_param("offset", "0"))
            .and(query_param("count", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_files))
            .mount(&server)
            .await;

        let params = GetSharedFilesListParams {
            order: Some("desc".to_string()),
            offset: Some(0),
            count: Some(20),
        };

        let result = file_api
            .get_shared_files_list(project_id, dir_path, params)
            .await;
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "test.txt");
        match &files[0].content {
            crate::models::FileContent::File { size } => assert_eq!(*size, 1024),
            _ => panic!("Expected file content"),
        }
        assert_eq!(files[0].project_id.value(), 123);
    }

    #[tokio::test]
    async fn test_get_shared_files_list_empty() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let file_api = FileApi::new(client);
        let project_key = "TEST_PROJECT";
        let dir_path = "empty";

        let expected_files: Vec<crate::models::SharedFile> = Vec::new();

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/files/metadata/{}",
                project_key, dir_path
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_files))
            .mount(&server)
            .await;

        let params = GetSharedFilesListParams::default();
        let result = file_api
            .get_shared_files_list(
                ProjectIdOrKey::from_str(project_key).unwrap(),
                dir_path,
                params,
            )
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_shared_files_list_project_not_found() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let file_api = FileApi::new(client);
        let project_id = 999;
        let dir_path = "documents";

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
            .and(path(format!(
                "/api/v2/projects/{}/files/metadata/{}",
                project_id, dir_path
            )))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = GetSharedFilesListParams::default();
        let result = file_api
            .get_shared_files_list(ProjectId::new(project_id), dir_path, params)
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
    async fn test_get_shared_files_list_with_custom_params() {
        let server = MockServer::start().await;
        let client = setup_client(&server).await;
        let file_api = FileApi::new(client);
        let project_id = ProjectId::new(456);
        let dir_path = "uploads";

        let expected_files: Vec<crate::models::SharedFile> = Vec::new();

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v2/projects/{}/files/metadata/{}",
                project_id, dir_path
            )))
            .and(query_param("order", "asc"))
            .and(query_param("offset", "10"))
            .and(query_param("count", "50"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_files))
            .mount(&server)
            .await;

        let params = GetSharedFilesListParams {
            order: Some("asc".to_string()),
            offset: Some(10),
            count: Some(50),
        };

        let result = file_api
            .get_shared_files_list(project_id, dir_path, params)
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
