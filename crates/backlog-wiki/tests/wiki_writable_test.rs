#[cfg(feature = "writable")]
mod writable_tests {
    use super::common::*;
    use backlog_core::identifier::{AttachmentId, SharedFileId, WikiAttachmentId, WikiId};
    use backlog_wiki::api::{
        AddRecentlyViewedWikiParams, AddWikiParams, AttachFilesToWikiParams,
        DeleteWikiAttachmentParams, DeleteWikiParams, LinkSharedFilesToWikiParams,
        UnlinkSharedFileFromWikiParams, UpdateWikiParams,
    };
    use wiremock::MockServer;
    use wiremock::matchers::{body_string_contains, header, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn test_update_wiki_success_with_all_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(123, 456, "Updated Wiki Title");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/123"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("name=Updated+Wiki+Title"))
            .and(body_string_contains("content=Updated+wiki+content"))
            .and(body_string_contains("mailNotify=true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(123))
            .name("Updated Wiki Title")
            .content("Updated wiki content")
            .mail_notify(true);

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "Updated Wiki Title");
        assert_eq!(detail.id.value(), 123);
    }

    #[tokio::test]
    async fn test_update_wiki_success_with_name_only() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(456, 789, "New Title Only");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/456"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("name=New+Title+Only"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(456)).name("New Title Only");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "New Title Only");
    }

    #[tokio::test]
    async fn test_update_wiki_success_with_content_only() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(789, 123, "Original Title");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/789"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("content=New+content+here"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(789)).content("New content here");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_wiki_success_with_mail_notify_false() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(111, 222, "Test Wiki");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/111"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("mailNotify=false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(111)).mail_notify(false);

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/404"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(404)).name("Does not exist");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/403"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(403)).name("Unauthorized");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/500"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(500)).content("Server error content");

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_wiki_empty_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(555, 666, "Unchanged");

        Mock::given(method("PATCH"))
            .and(path("/api/v2/wikis/555"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = UpdateWikiParams::new(WikiId::new(555));

        let result = wiki_api.update_wiki(params).await;
        assert!(result.is_ok());
    }

    // Tests for add_wiki functionality
    #[tokio::test]
    async fn test_add_wiki_success_with_all_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(789, 123, "New Wiki Page");

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=123"))
            .and(body_string_contains("name=New+Wiki+Page"))
            .and(body_string_contains("content=This+is+new+wiki+content"))
            .and(body_string_contains("mailNotify=true"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(
            ProjectId::new(123),
            "New Wiki Page",
            "This is new wiki content",
        )
        .mail_notify(true);

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "New Wiki Page");
        assert_eq!(detail.project_id.value(), 123);
    }

    #[tokio::test]
    async fn test_add_wiki_success_minimal_params() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(456, 789, "Minimal Wiki");

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=789"))
            .and(body_string_contains("name=Minimal+Wiki"))
            .and(body_string_contains("content=Basic+content"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(789), "Minimal Wiki", "Basic content");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "Minimal Wiki");
    }

    #[tokio::test]
    async fn test_add_wiki_success_with_mail_notify_false() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(111, 222, "No Notify Wiki");

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("projectId=222"))
            .and(body_string_contains("name=No+Notify+Wiki"))
            .and(body_string_contains("content=Content+without+notification"))
            .and(body_string_contains("mailNotify=false"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(
            ProjectId::new(222),
            "No Notify Wiki",
            "Content without notification",
        )
        .mail_notify(false);

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_wiki_bad_request() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(400), "Bad Request", "Invalid content");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(403), "Unauthorized", "No permission");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_wiki_project_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = AddWikiParams::new(ProjectId::new(404), "Not Found", "Project does not exist");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params =
            AddWikiParams::new(ProjectId::new(500), "Server Error", "Internal server error");

        let result = wiki_api.add_wiki(params).await;
        assert!(result.is_err());
    }

    // Tests for delete_wiki functionality
    #[tokio::test]
    async fn test_delete_wiki_success() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(123, 456, "Deleted Wiki Page");

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiParams::new(WikiId::new(123));

        let result = wiki_api.delete_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "Deleted Wiki Page");
        assert_eq!(detail.id.value(), 123);
    }

    #[tokio::test]
    async fn test_delete_wiki_success_with_mail_notify() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(456, 789, "Deleted with Notification");

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/456"))
            .and(query_param("mailNotify", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiParams::new(WikiId::new(456)).mail_notify(true);

        let result = wiki_api.delete_wiki(params).await;
        assert!(result.is_ok());
        let detail = result.unwrap();
        assert_eq!(detail.name, "Deleted with Notification");
    }

    #[tokio::test]
    async fn test_delete_wiki_success_with_mail_notify_false() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_detail = create_mock_wiki_detail(789, 123, "Deleted without Notification");

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/789"))
            .and(query_param("mailNotify", "false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_detail))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiParams::new(WikiId::new(789)).mail_notify(false);

        let result = wiki_api.delete_wiki(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/404"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiParams::new(WikiId::new(404));

        let result = wiki_api.delete_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/403"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiParams::new(WikiId::new(403));

        let result = wiki_api.delete_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/500"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiParams::new(WikiId::new(500));

        let result = wiki_api.delete_wiki(params).await;
        assert!(result.is_err());
    }

    // Tests for attach_files_to_wiki functionality
    #[tokio::test]
    async fn test_attach_files_to_wiki_success_single_file() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_attachment = create_mock_wiki_attachment(456, "document.pdf", 2048, 1, "john");
        let expected_response = vec![expected_attachment];

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/123/attachments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AttachFilesToWikiParams::new(WikiId::new(123), vec![AttachmentId::new(456)]);

        let result = wiki_api.attach_files_to_wiki(params).await;
        match result {
            Ok(attachments) => {
                assert_eq!(attachments.len(), 1);
                assert_eq!(attachments[0].id.value(), 456);
                assert_eq!(attachments[0].name, "document.pdf");
            }
            Err(e) => {
                panic!("Expected Ok but got error: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_attach_files_to_wiki_success_multiple_files() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_attachments = vec![
            create_mock_wiki_attachment(111, "file1.txt", 1024, 1, "alice"),
            create_mock_wiki_attachment(222, "file2.jpg", 3072, 2, "bob"),
            create_mock_wiki_attachment(333, "file3.pdf", 4096, 3, "charlie"),
        ];

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/789/attachments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_attachments))
            .mount(&mock_server)
            .await;

        let params = AttachFilesToWikiParams::new(
            WikiId::new(789),
            vec![
                AttachmentId::new(111),
                AttachmentId::new(222),
                AttachmentId::new(333),
            ],
        );

        let result = wiki_api.attach_files_to_wiki(params).await;
        match result {
            Ok(attachments) => {
                assert_eq!(attachments.len(), 3);
                assert_eq!(attachments[0].name, "file1.txt");
                assert_eq!(attachments[1].name, "file2.jpg");
                assert_eq!(attachments[2].name, "file3.pdf");
            }
            Err(e) => {
                panic!("Expected Ok but got error: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_attach_files_to_wiki_success_empty_attachments() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_response: Vec<WikiAttachment> = vec![];

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/555/attachments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = AttachFilesToWikiParams::new(WikiId::new(555), vec![]);

        let result = wiki_api.attach_files_to_wiki(params).await;
        assert!(result.is_ok());
        let attachments = result.unwrap();
        assert_eq!(attachments.len(), 0);
    }

    #[tokio::test]
    async fn test_attach_files_to_wiki_bad_request() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/400/attachments"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;

        let params = AttachFilesToWikiParams::new(WikiId::new(400), vec![AttachmentId::new(999)]);

        let result = wiki_api.attach_files_to_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_attach_files_to_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/403/attachments"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let params = AttachFilesToWikiParams::new(WikiId::new(403), vec![AttachmentId::new(123)]);

        let result = wiki_api.attach_files_to_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_attach_files_to_wiki_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/404/attachments"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = AttachFilesToWikiParams::new(WikiId::new(404), vec![AttachmentId::new(456)]);

        let result = wiki_api.attach_files_to_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_attach_files_to_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/500/attachments"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = AttachFilesToWikiParams::new(WikiId::new(500), vec![AttachmentId::new(789)]);

        let result = wiki_api.attach_files_to_wiki(params).await;
        assert!(result.is_err());
    }

    // Tests for delete_wiki_attachment functionality
    #[tokio::test]
    async fn test_delete_wiki_attachment_success() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_attachment =
            create_mock_wiki_attachment(789, "deleted-file.pdf", 2048, 1, "user1");

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/123/attachments/789"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachment))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiAttachmentParams::new(WikiId::new(123), WikiAttachmentId::new(789));

        let result = wiki_api.delete_wiki_attachment(params).await;
        assert!(result.is_ok());
        let attachment = result.unwrap();
        assert_eq!(attachment.id.value(), 789);
        assert_eq!(attachment.name, "deleted-file.pdf");
    }

    #[tokio::test]
    async fn test_delete_wiki_attachment_returns_details() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_attachment =
            create_mock_wiki_attachment(456, "document.txt", 1024, 2, "admin");

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/555/attachments/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_attachment))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiAttachmentParams::new(WikiId::new(555), WikiAttachmentId::new(456));

        let result = wiki_api.delete_wiki_attachment(params).await;
        assert!(result.is_ok());
        let attachment = result.unwrap();
        assert_eq!(attachment.name, "document.txt");
        assert_eq!(attachment.size, 1024);
    }

    #[tokio::test]
    async fn test_delete_wiki_attachment_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/404/attachments/123"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiAttachmentParams::new(WikiId::new(404), WikiAttachmentId::new(123));

        let result = wiki_api.delete_wiki_attachment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_wiki_attachment_attachment_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/123/attachments/404"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiAttachmentParams::new(WikiId::new(123), WikiAttachmentId::new(404));

        let result = wiki_api.delete_wiki_attachment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_wiki_attachment_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/403/attachments/123"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiAttachmentParams::new(WikiId::new(403), WikiAttachmentId::new(123));

        let result = wiki_api.delete_wiki_attachment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_wiki_attachment_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/500/attachments/123"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiAttachmentParams::new(WikiId::new(500), WikiAttachmentId::new(123));

        let result = wiki_api.delete_wiki_attachment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_wiki_attachment_bad_request() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/400/attachments/123"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;

        let params = DeleteWikiAttachmentParams::new(WikiId::new(400), WikiAttachmentId::new(123));

        let result = wiki_api.delete_wiki_attachment(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_link_shared_files_to_wiki_success_single_file() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_shared_files = vec![create_mock_shared_file(
            123,
            456,
            "/docs",
            "document.pdf",
            2048,
            1,
            "alice",
        )];

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/789/sharedFiles"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("fileId%5B%5D=123")) // fileId[]=123 URL encoded
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
            .mount(&mock_server)
            .await;

        let params =
            LinkSharedFilesToWikiParams::new(WikiId::new(789), vec![SharedFileId::new(123)]);

        let result = wiki_api.link_shared_files_to_wiki(params).await;
        assert!(result.is_ok());
        let shared_files = result.unwrap();
        assert_eq!(shared_files.len(), 1);
        assert_eq!(shared_files[0].name, "document.pdf");
        assert_eq!(shared_files[0].id.value(), 123);
    }

    #[tokio::test]
    async fn test_link_shared_files_to_wiki_success_multiple_files() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_shared_files = vec![
            create_mock_shared_file(111, 456, "/docs", "file1.pdf", 1024, 1, "alice"),
            create_mock_shared_file(222, 456, "/images", "file2.png", 2048, 2, "bob"),
            create_mock_shared_file(333, 456, "/data", "file3.xlsx", 4096, 1, "alice"),
        ];

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/456/sharedFiles"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("fileId%5B%5D=111"))
            .and(body_string_contains("fileId%5B%5D=222"))
            .and(body_string_contains("fileId%5B%5D=333"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
            .mount(&mock_server)
            .await;

        let params = LinkSharedFilesToWikiParams::new(
            WikiId::new(456),
            vec![
                SharedFileId::new(111),
                SharedFileId::new(222),
                SharedFileId::new(333),
            ],
        );

        let result = wiki_api.link_shared_files_to_wiki(params).await;
        assert!(result.is_ok());
        let shared_files = result.unwrap();
        assert_eq!(shared_files.len(), 3);
        assert_eq!(shared_files[0].name, "file1.pdf");
        assert_eq!(shared_files[1].name, "file2.png");
        assert_eq!(shared_files[2].name, "file3.xlsx");
    }

    #[tokio::test]
    async fn test_link_shared_files_to_wiki_empty_files_list() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_shared_files: Vec<SharedFile> = vec![];

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/123/sharedFiles"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_files))
            .mount(&mock_server)
            .await;

        let params = LinkSharedFilesToWikiParams::new(WikiId::new(123), vec![]);

        let result = wiki_api.link_shared_files_to_wiki(params).await;
        assert!(result.is_ok());
        let shared_files = result.unwrap();
        assert_eq!(shared_files.len(), 0);
    }

    #[tokio::test]
    async fn test_link_shared_files_to_wiki_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/999/sharedFiles"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Wiki not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params =
            LinkSharedFilesToWikiParams::new(WikiId::new(999), vec![SharedFileId::new(123)]);

        let result = wiki_api.link_shared_files_to_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_link_shared_files_to_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/999/sharedFiles"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [{"message": "You do not have permission to link files to this wiki"}]
            })))
            .mount(&mock_server)
            .await;

        let params =
            LinkSharedFilesToWikiParams::new(WikiId::new(999), vec![SharedFileId::new(456)]);

        let result = wiki_api.link_shared_files_to_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_link_shared_files_to_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/wikis/123/sharedFiles"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "errors": [{"message": "Internal server error"}]
            })))
            .mount(&mock_server)
            .await;

        let params =
            LinkSharedFilesToWikiParams::new(WikiId::new(123), vec![SharedFileId::new(789)]);

        let result = wiki_api.link_shared_files_to_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlink_shared_file_from_wiki_success() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_shared_file =
            create_mock_shared_file(456, 123, "/docs", "removed.pdf", 1024, 2, "alice");

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/123/sharedFiles/456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_shared_file))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileFromWikiParams::new(WikiId::new(123), SharedFileId::new(456));

        let result = wiki_api.unlink_shared_file_from_wiki(params).await;
        assert!(result.is_ok());
        let shared_file = result.unwrap();
        assert_eq!(shared_file.name, "removed.pdf");
        assert_eq!(shared_file.id.value(), 456);
    }

    #[tokio::test]
    async fn test_unlink_shared_file_from_wiki_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/999/sharedFiles/123"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Wiki not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileFromWikiParams::new(WikiId::new(999), SharedFileId::new(123));

        let result = wiki_api.unlink_shared_file_from_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlink_shared_file_from_wiki_shared_file_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/123/sharedFiles/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Shared file not linked to this wiki"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileFromWikiParams::new(WikiId::new(123), SharedFileId::new(999));

        let result = wiki_api.unlink_shared_file_from_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlink_shared_file_from_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/123/sharedFiles/456"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "errors": [{"message": "You do not have permission to unlink files from this wiki"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileFromWikiParams::new(WikiId::new(123), SharedFileId::new(456));

        let result = wiki_api.unlink_shared_file_from_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlink_shared_file_from_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/api/v2/wikis/123/sharedFiles/456"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "errors": [{"message": "Internal server error"}]
            })))
            .mount(&mock_server)
            .await;

        let params = UnlinkSharedFileFromWikiParams::new(WikiId::new(123), SharedFileId::new(456));

        let result = wiki_api.unlink_shared_file_from_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_recently_viewed_wiki_success() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        let expected_wiki = serde_json::json!({
            "id": 123,
            "projectId": 456,
            "name": "Recently Viewed Wiki",
            "tags": [
                {"id": 1, "name": "important"}
            ],
            "createdUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "created": "2024-01-01T09:00:00Z",
            "updatedUser": {
                "id": 1,
                "userId": "admin",
                "name": "Admin User",
                "roleType": 1,
                "lang": "ja",
                "mailAddress": "admin@example.com"
            },
            "updated": "2024-01-15T10:30:00Z"
        });

        Mock::given(method("POST"))
            .and(path("/api/v2/users/myself/recentlyViewedWikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("wikiId=123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_wiki))
            .mount(&mock_server)
            .await;

        let params = AddRecentlyViewedWikiParams {
            wiki_id: WikiId::new(123),
        };

        let result = wiki_api.add_recently_viewed_wiki(params).await;
        assert!(result.is_ok());
        let wiki = result.unwrap();
        assert_eq!(wiki.id.value(), 123);
        assert_eq!(wiki.name, "Recently Viewed Wiki");
        assert_eq!(wiki.project_id.value(), 456);
    }

    #[tokio::test]
    async fn test_add_recently_viewed_wiki_not_found() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/users/myself/recentlyViewedWikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("wikiId=999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "Wiki not found"}]
            })))
            .mount(&mock_server)
            .await;

        let params = AddRecentlyViewedWikiParams {
            wiki_id: WikiId::new(999),
        };

        let result = wiki_api.add_recently_viewed_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_recently_viewed_wiki_unauthorized() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/users/myself/recentlyViewedWikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("wikiId=123"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "errors": [{"message": "Authentication required"}]
            })))
            .mount(&mock_server)
            .await;

        let params = AddRecentlyViewedWikiParams {
            wiki_id: WikiId::new(123),
        };

        let result = wiki_api.add_recently_viewed_wiki(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_recently_viewed_wiki_server_error() {
        let mock_server = MockServer::start().await;
        let wiki_api = setup_wiki_api(&mock_server).await;

        Mock::given(method("POST"))
            .and(path("/api/v2/users/myself/recentlyViewedWikis"))
            .and(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(body_string_contains("wikiId=123"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let params = AddRecentlyViewedWikiParams {
            wiki_id: WikiId::new(123),
        };

        let result = wiki_api.add_recently_viewed_wiki(params).await;
        assert!(result.is_err());
    }
}

mod common;
