#[cfg(feature = "writable")]
mod writable_tests {
    use backlog_space::api::{SpaceApi, UploadAttachmentParams};
    use client::test_utils::setup_client;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_space_api(mock_server: &MockServer) -> SpaceApi {
        let client = setup_client(mock_server).await;
        SpaceApi::new(client)
    }

    #[tokio::test]
    async fn test_upload_attachment_success() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        // Create a temporary test file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = b"test file content for attachment";
        fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

        let mock_response = serde_json::json!({
            "id": 456,
            "name": "test_attachment.txt",
            "size": 32
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/space/attachment"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&server)
            .await;

        let params = UploadAttachmentParams::new(temp_file.path().to_path_buf());
        let result = space_api.upload_attachment(params).await;

        assert!(result.is_ok());
        let attachment = result.unwrap();
        assert_eq!(attachment.id, 456);
        assert_eq!(attachment.name, "test_attachment.txt");
        assert_eq!(attachment.size, 32);
    }

    #[tokio::test]
    async fn test_upload_attachment_file_not_found() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        let non_existent_file = PathBuf::from("/tmp/non_existent_attachment.txt");
        let params = UploadAttachmentParams::new(non_existent_file);

        let result = space_api.upload_attachment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upload_attachment_api_error() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        // Create a temporary test file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = b"large file content";
        fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "File size too large",
                    "code": 2,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/space/attachment"))
            .respond_with(ResponseTemplate::new(413).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UploadAttachmentParams::new(temp_file.path().to_path_buf());
        let result = space_api.upload_attachment(params).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upload_attachment_unauthorized() {
        let server = MockServer::start().await;
        let space_api = setup_space_api(&server).await;

        // Create a temporary test file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_content = b"test content";
        fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Unauthorized access",
                    "code": 1,
                    "moreInfo": ""
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/space/attachment"))
            .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
            .mount(&server)
            .await;

        let params = UploadAttachmentParams::new(temp_file.path().to_path_buf());
        let result = space_api.upload_attachment(params).await;

        assert!(result.is_err());
    }
}
