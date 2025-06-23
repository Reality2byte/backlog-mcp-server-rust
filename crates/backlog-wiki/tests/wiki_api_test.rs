mod common;
use common::*;

use backlog_core::ProjectKey;
use backlog_wiki::WikiCount;
use backlog_wiki::api::{
    DownloadWikiAttachmentParams, GetWikiAttachmentListParams, GetWikiCountParams,
    GetWikiDetailParams, GetWikiListParams,
};
use wiremock::MockServer;
use wiremock::matchers::{method, path, query_param};

#[tokio::test]
async fn test_get_wiki_list_empty_params_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_wikis = vec![
        create_mock_wiki(112, 103, "Home", 1, "john"),
        create_mock_wiki(113, 103, "Documentation", 2, "alice"),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
        .mount(&mock_server)
        .await;

    let params = GetWikiListParams::new();
    let result = wiki_api.get_wiki_list(params).await;
    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 2);
    assert_eq!(wikis[0].name, "Home");
    assert_eq!(wikis[1].name, "Documentation");
}

#[tokio::test]
async fn test_get_wiki_list_with_project_id() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_wikis = vec![create_mock_wiki(112, 123, "Home", 1, "john")];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis"))
        .and(query_param("projectIdOrKey", "MYPROJECT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
        .mount(&mock_server)
        .await;

    let params =
        GetWikiListParams::new().project_id_or_key("MYPROJECT".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_list(params).await;
    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 1);
    assert_eq!(wikis[0].name, "Home");
}

#[tokio::test]
async fn test_get_wiki_list_with_keyword() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_wikis = vec![create_mock_wiki(113, 103, "Documentation", 2, "alice")];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis"))
        .and(query_param("keyword", "doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wikis))
        .mount(&mock_server)
        .await;

    let params = GetWikiListParams::new().keyword("doc");
    let result = wiki_api.get_wiki_list(params).await;
    assert!(result.is_ok());
    let wikis = result.unwrap();
    assert_eq!(wikis.len(), 1);
    assert_eq!(wikis[0].name, "Documentation");
}

#[tokio::test]
async fn test_get_wiki_count_without_project() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_count = WikiCount { count: 42 };

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/count"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_count))
        .mount(&mock_server)
        .await;

    let params = GetWikiCountParams::new();
    let result = wiki_api.get_wiki_count(params).await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 42);
}

#[tokio::test]
async fn test_get_wiki_count_with_project() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_count = WikiCount { count: 15 };

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/count"))
        .and(query_param("projectIdOrKey", "MYPROJECT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_count))
        .mount(&mock_server)
        .await;

    let params =
        GetWikiCountParams::new().project_id_or_key("MYPROJECT".parse::<ProjectKey>().unwrap());
    let result = wiki_api.get_wiki_count(params).await;
    assert!(result.is_ok());
    let count = result.unwrap();
    assert_eq!(count.count, 15);
}

#[tokio::test]
async fn test_get_wiki_detail_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_detail = create_mock_wiki_detail(123, 456, "API Documentation");

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(WikiId::new(123)))
        .await;
    assert!(result.is_ok());
    let detail = result.unwrap();
    assert_eq!(detail.id.value(), 123);
    assert_eq!(detail.project_id.value(), 456);
    assert_eq!(detail.name, "API Documentation");
    assert!(detail.content.contains("API Documentation"));
    assert_eq!(detail.tags.len(), 1);
    assert_eq!(detail.attachments.len(), 1);
    assert_eq!(detail.shared_files.len(), 1);
    assert_eq!(detail.stars.len(), 1);
}

#[tokio::test]
async fn test_get_wiki_detail_with_u32_id() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_detail = create_mock_wiki_detail(789, 101, "User Guide");

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/789"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_detail(GetWikiDetailParams::new(789u32))
        .await;
    assert!(result.is_ok());
    let detail = result.unwrap();
    assert_eq!(detail.id.value(), 789);
    assert_eq!(detail.name, "User Guide");
}

#[tokio::test]
async fn test_get_wiki_attachment_list_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let expected_attachments = vec![
        create_mock_wiki_attachment(1, "document.pdf", 1024, 1, "john"),
        create_mock_wiki_attachment(2, "image.png", 2048, 2, "alice"),
        create_mock_wiki_attachment(3, "spreadsheet.xlsx", 4096, 1, "john"),
    ];

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/attachments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachments))
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .get_wiki_attachment_list(GetWikiAttachmentListParams::new(WikiId::new(123)))
        .await;
    assert!(result.is_ok());
    let attachments = result.unwrap();
    assert_eq!(attachments.len(), 3);
    assert_eq!(attachments[0].name, "document.pdf");
    assert_eq!(attachments[0].size, 1024);
    assert_eq!(attachments[1].name, "image.png");
    assert_eq!(attachments[1].size, 2048);
    assert_eq!(attachments[2].name, "spreadsheet.xlsx");
    assert_eq!(attachments[2].size, 4096);
}

#[tokio::test]
async fn test_download_wiki_attachment_success() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let attachment_content = "This is a test attachment content.";

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/123/attachments/456"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(attachment_content)
                .insert_header("Content-Type", "application/octet-stream")
                .insert_header("Content-Disposition", "attachment; filename=\"test.txt\""),
        )
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .download_wiki_attachment(DownloadWikiAttachmentParams::new(
            WikiId::new(123),
            WikiAttachmentId::new(456),
        ))
        .await;
    assert!(result.is_ok());
    let downloaded_file = result.unwrap();
    assert_eq!(downloaded_file.filename, "test.txt");
    assert_eq!(downloaded_file.content_type, "application/octet-stream");
    assert_eq!(downloaded_file.bytes.len(), attachment_content.len());
}

#[tokio::test]
async fn test_download_wiki_attachment_with_u32_ids() {
    let mock_server = MockServer::start().await;
    let wiki_api = setup_wiki_api(&mock_server).await;

    let attachment_content = "Test content for u32 ID test.";

    Mock::given(method("GET"))
        .and(path("/api/v2/wikis/789/attachments/101"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(attachment_content)
                .insert_header("Content-Type", "image/png")
                .insert_header("Content-Disposition", "attachment; filename=\"image.png\""),
        )
        .mount(&mock_server)
        .await;

    let result = wiki_api
        .download_wiki_attachment(DownloadWikiAttachmentParams::new(789u32, 101u32))
        .await;
    assert!(result.is_ok());
    let downloaded_file = result.unwrap();
    assert_eq!(downloaded_file.filename, "image.png");
    assert_eq!(downloaded_file.content_type, "image/png");
    assert_eq!(downloaded_file.bytes.len(), attachment_content.len());
}
