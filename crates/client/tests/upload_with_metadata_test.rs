use backlog_api_core::IntoUploadRequest;
use client::Client;
use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Debug, Clone)]
struct UploadWithAdditionalFields {
    file_path: PathBuf,
    project_id: String,
    comment: String,
}

impl IntoUploadRequest for UploadWithAdditionalFields {
    fn path(&self) -> String {
        "/api/v2/files/upload".to_string()
    }

    fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    fn file_field_name(&self) -> &str {
        "document"
    }

    fn additional_fields(&self) -> Vec<(String, String)> {
        vec![
            ("projectId".to_string(), self.project_id.clone()),
            ("comment".to_string(), self.comment.clone()),
        ]
    }
}

#[tokio::test]
async fn test_upload_file_with_metadata() {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri()).expect("Failed to create client");

    // Create a temporary test file
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_content = b"document content";
    fs::write(temp_file.path(), test_content).expect("Failed to write to temp file");

    let mock_response = serde_json::json!({
        "id": 456,
        "name": "document.pdf",
        "size": 16,
        "projectId": "PROJ123"
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/files/upload"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&server)
        .await;

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct UploadResponse {
        id: u32,
        name: String,
        size: u64,
        #[serde(rename = "projectId")]
        project_id: String,
    }

    let params = UploadWithAdditionalFields {
        file_path: temp_file.path().to_path_buf(),
        project_id: "PROJ123".to_string(),
        comment: "Initial upload".to_string(),
    };

    let result: Result<UploadResponse, _> = client.upload_file(params).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.id, 456);
    assert_eq!(response.name, "document.pdf");
    assert_eq!(response.size, 16);
    assert_eq!(response.project_id, "PROJ123");
}
