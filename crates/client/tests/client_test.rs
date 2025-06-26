use backlog_api_core::IntoUploadRequest;
use client::Client;
use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Debug, Clone)]
struct TestUploadParams {
    file_path: PathBuf,
}

impl IntoUploadRequest for TestUploadParams {
    fn path(&self) -> String {
        "/api/v2/space/attachment".to_string()
    }

    fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

#[tokio::test]
async fn test_upload_file_success() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Failed to create client");

    // Create a temporary test file
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_content = b"test file content";
    fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

    let mock_response = serde_json::json!({
        "id": 123,
        "name": "test.txt",
        "size": 17
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/space/attachment"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct AttachmentResponse {
        id: u32,
        name: String,
        size: u64,
    }

    let params = TestUploadParams {
        file_path: temp_file.path().to_path_buf(),
    };
    let result: Result<AttachmentResponse, _> = client.upload_file(params).await;
    assert!(result.is_ok());
    let attachment = result.unwrap();
    assert_eq!(attachment.id, 123);
    assert_eq!(attachment.name, "test.txt");
    assert_eq!(attachment.size, 17);
}

#[tokio::test]
async fn test_upload_file_not_found() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Failed to create client");

    let non_existent_file = PathBuf::from("/tmp/non_existent_file.txt");

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct AttachmentResponse {
        id: u32,
        name: String,
        size: u64,
    }

    let params = TestUploadParams {
        file_path: non_existent_file,
    };
    let result: Result<AttachmentResponse, _> = client.upload_file(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_upload_file_api_error() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Failed to create client");

    // Create a temporary test file
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_content = b"test file content";
    fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

    let error_response = serde_json::json!({
        "errors": [
            {
                "message": "File too large",
                "code": 1,
                "moreInfo": ""
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/space/attachment"))
        .respond_with(ResponseTemplate::new(413).set_body_json(&error_response))
        .mount(&server)
        .await;

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct AttachmentResponse {
        id: u32,
        name: String,
        size: u64,
    }

    let params = TestUploadParams {
        file_path: temp_file.path().to_path_buf(),
    };
    let result: Result<AttachmentResponse, _> = client.upload_file(params).await;
    assert!(result.is_err());
}
